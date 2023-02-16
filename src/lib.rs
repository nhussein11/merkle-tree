use sha2::Digest;
use std::cell::RefCell;
use std::rc::Rc;


mod tests;
pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

#[derive(Debug)]
pub struct MerkleTree {
    nodes: Vec<MerkleNode>,
}

#[derive(Debug, Clone)]
pub struct MerkleNode {
    parent: Option<Rc<RefCell<MerkleNode>>>,
    hash : Hash,
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

        
        let mut nodes : Vec<MerkleNode> = input
                .iter()
                .map(|leaf| MerkleNode{parent: None, hash: hash_data(leaf)})
                .collect::<Vec<MerkleNode>>();

        let mut nodes_level = nodes.clone();

        for level in 0..levels {
             nodes_level.chunks(2).for_each(|chunk| {
                let mut left_child = chunk[0].clone();
                let mut right_child = chunk[1].clone();
            
                let parent_hash = concatenate_and_hash(left_child.clone().hash, right_child.clone().hash);
                let parent = MerkleNode{parent: None, hash: parent_hash};
            
                update_child_node(&mut left_child, parent.clone());
                update_child_node(&mut right_child, parent.clone());

                nodes.push(parent);
            });

            nodes_level = nodes[nodes.len() - nodes_level.len() / 2..].to_vec();
        }

        println!("Nodes: {:?}", nodes);

        MerkleTree { nodes }
    }

//    /// Check if the given data produces the given root hash
//    pub fn verify(input: &[Data], root: Hash) -> bool {
//        let merkle_tree = MerkleTree::construct_by_input(input);
//        merkle_tree.root_hash() == root
//    }
//
//    /// Returns the root hash of the MerkleTree
//    pub fn root_hash(&self) -> Hash {
//        self.nodes.last().unwrap().clone()
//    }
//
//    /// Returns the merkle proof for the given leaf index
//    pub fn get_merkle_proof_by_leaf_index(&self, index: usize) -> Result<Vec<Hash>, Error> {
//        if index >= self.get_number_of_leaves() {
//            return Err(Error::InvalidLeafIndex);
//        }
//
//        let mut proof = Vec::new();
//        let mut index = index;
//
//        for level in 0..self.levels {
//            let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };
//
//            proof.push(self.nodes[sibling_index].clone());
//            index = index / 2;
//        }
//
//        Ok(proof)
//    }
//
//    /// Returns the merkle proof for the given data
//    pub fn get_merkle_proof_by_data(&self, data: Hash) -> Result<Vec<Hash>, Error> {
//        let data_hashed = hash_data(&data);
//        self.leaves()
//            .iter()
//            .position(|leaf| leaf == &data_hashed)
//            .map(|index| self.get_merkle_proof_by_leaf_index(index))
//            .unwrap_or(Err(Error::InvalidData))
//    }
//
//    /// Get number of leaves in the MerkleTree
//    pub fn get_number_of_leaves(&self) -> usize {
//        2i32.pow(self.levels as u32).try_into().unwrap()
//    }
//
//    /// Get leaves of the MerkleTree
//    pub fn leaves(&self) -> &[Hash] {
//        &self.nodes[0..self.get_number_of_leaves()]
//    }
}

/// Helper functions
/// Checks if the given number is power of two
fn is_power_of_two(n: usize) -> bool {
    n != 0 && n & (n - 1) == 0
}
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
/// Hashes the given data using sha256
fn hash_data(data: &Data) -> Hash {
    sha2::Sha256::digest(data).to_vec()
}
/// Concatenates the given hashes and returns the hash of the concatenated data
fn concatenate_and_hash(left: Hash, right: Hash) -> Hash {
    let parent_hash = [left, right].concat();
    hash_data(&parent_hash)
}
/// Update child node to point to parent
fn update_child_node(child: &mut MerkleNode, parent: MerkleNode) {
    child.parent = Some(Rc::new(RefCell::new(parent)));
}
