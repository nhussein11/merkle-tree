use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as StdHash, Hasher};
use std::rc::Rc;

mod tests;
pub type Data = Vec<u8>;
pub type Hash = String;

#[derive(Debug)]
pub struct MerkleTree {
    nodes: Vec<MerkleNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleNode {
    parent: Option<Rc<RefCell<MerkleNode>>>,
    hash: Hash,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidLeafIndex,
    InvalidData,
}

impl MerkleTree {
    /// Constructs a MerkleTree from the given input
    pub fn construct_by_input(input: &[Data]) -> Self {
        let input = pair_off_tree(input);
        let levels = input.len().trailing_zeros() as usize;

        let mut nodes: Vec<MerkleNode> = input
            .iter()
            .map(|leaf| MerkleNode {
                parent: None,
                hash: hash_data(leaf),
            })
            .collect::<Vec<MerkleNode>>();

        let mut nodes_level = nodes.clone();

        for _level in 0..levels {
            nodes_level.chunks_mut(2).for_each(|chunk| {
                let parent_hash = concatenate_and_hash(&chunk[0].hash, &chunk[1].hash);

                let parent = MerkleNode {
                    parent: None,
                    hash: parent_hash,
                };

                nodes.push(parent.clone());

                let left_child_position = get_node_index(&nodes, &chunk[0].hash).unwrap();
                let right_child_position = get_node_index(&nodes, &chunk[1].hash).unwrap();

                nodes[left_child_position].update_parent(&parent);
                nodes[right_child_position].update_parent(&parent);
            });
            nodes_level = nodes[nodes.len() - nodes_level.len() / 2..].to_vec();
        }

        MerkleTree { nodes }
    }

    /// Check if the given data produces the given root hash
    pub fn verify(input: &[Data], root: Hash) -> bool {
        let merkle_tree = MerkleTree::construct_by_input(input);
        merkle_tree.root_hash() == root
    }

    /// Returns the root hash of the MerkleTree
    pub fn root_hash(&self) -> Hash {
        self.nodes.last().unwrap().hash.clone()
    }

    /// Returns the merkle proof for the given leaf index
    pub fn get_merkle_proof_by_leaf_index(&self, index: usize) -> Result<Hash, Error> {
        if index >= self.get_number_of_leaves() {
            return Err(Error::InvalidLeafIndex);
        }

        let mut current_node = &self.nodes[index];
        let mut proof = current_node.hash.clone();

        while let Some(parent) = &current_node.parent {
            let current_node_index = get_node_index(&self.nodes, &current_node.hash).unwrap();

            let sibling_index = if current_node_index % 2 == 0 {
                current_node_index + 1
            } else {
                current_node_index - 1
            };

            let node = &self.nodes[current_node_index];
            let sibling = &self.nodes[sibling_index];
            proof = concatenate_and_hash(&node.hash, &sibling.hash);

            let parent_index = get_node_index(&self.nodes, &parent.borrow().hash).unwrap();

            current_node = &self.nodes[parent_index];
        }

        Ok(proof)
    }

    /// Returns the merkle proof for the given data
    pub fn get_merkle_proof_by_data(&self, data: Data) -> Result<Hash, Error> {
        let data_hashed = hash_data(&data);
        self.leaves()
            .iter()
            .position(|leaf| leaf.hash == data_hashed)
            .map(|index| self.get_merkle_proof_by_leaf_index(index))
            .unwrap_or(Err(Error::InvalidData))
    }

    /// Get number of leaves in the MerkleTree
    pub fn get_number_of_leaves(&self) -> usize {
        self.nodes.len() / 2 + 1
    }

    /// Get leaves of the MerkleTree
    pub fn leaves(&self) -> &[MerkleNode] {
        &self.nodes[0..self.get_number_of_leaves()]
    }

    /// Get levels of the MerkleTree
    pub fn levels(&self) -> usize {
        self.nodes.len().trailing_zeros() as usize
    }

    /// Get node index of the given hash
    pub fn get_node_index(&self, hash: Hash) -> Option<usize> {
        self.nodes.iter().position(|node| node.hash == hash)
    }
}

impl MerkleNode {
    fn update_parent(&mut self, parent: &MerkleNode) {
        self.parent = Some(Rc::new(RefCell::new(parent.clone())));
    }
}

/// Helper functions
/// Pads the input with empty data to make it power of two
fn pair_off_tree(input: &[Data]) -> Vec<Data> {
    match input.len().is_power_of_two() {
        true => input.to_vec(),
        false => {
            let padding_len = input.len().next_power_of_two() - input.len();
            let mut input = input.to_vec();
            input.reserve_exact(padding_len);
            for _ in 0..padding_len {
                input.push(vec![0u8; 32]);
            }
            input
        }
    }
}
/// Hashes the given data using Hash from std
fn hash_data(data: &Data) -> Hash {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish().to_string()
}
/// Concatenates the given hashes and returns the hash of the concatenated data
fn concatenate_and_hash(left: &Hash, right: &Hash) -> Hash {
    let parent_hash = format!("{}{}", left, right).into_bytes();
    hash_data(&parent_hash)
}
/// Get node index of the given hash
fn get_node_index(nodes: &[MerkleNode], hash: &Hash) -> Option<usize> {
    nodes.iter().position(|node| node.hash == *hash)
}
