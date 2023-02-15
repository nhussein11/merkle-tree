use sha2::Digest;

mod tests;
pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

pub struct MerkleTree {
    nodes: Vec<Hash>,
    levels: usize,
}

impl MerkleTree {
    /// Constructs a MerkleTree from the given input
    pub fn construct_by_input(input: &[Data]) -> Self {
        let input = pair_off_tree(input);
        let levels = input.len().trailing_zeros() as usize;

        let mut leaves = input
            .iter()
            .map(hash_data)
            .collect::<Vec<Hash>>();

        let mut nodes = leaves.clone();
       
        for level in 0..levels {
            leaves = leaves
                .chunks(2)
                .map(|chunk| concatenate_hashes(chunk[0].clone(), chunk[1].clone()))
                .collect::<Vec<Hash>>();

            nodes.extend(leaves.clone());
        }


        MerkleTree {
            nodes,
            levels
        }
    }
    /// Check if the given data produces the given root hash
    pub fn verify(input: &[Data], root: Hash) -> bool {
        todo!()
    }

    /// Returns the root hash of the MerkleTree
    pub fn root_hash(&self) -> Hash {
        todo!()
    }

    pub fn get_merkle_proof_by_leaf_index(index: usize) -> bool {
        todo!()
    }

    pub fn get_merkle_proof_by_data(data: Hash) {
        todo!()
    }
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
fn concatenate_hashes(left: Hash, right: Hash) -> Hash {
    let parent_hash = [left, right].concat();
    hash_data(&parent_hash)
}
