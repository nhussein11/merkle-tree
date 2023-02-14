
pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

pub struct MerkleTree {
    nodes: Vec<Hash>,
}

impl MerkleTree {
    fn build_merkle_tree(items: Vec<Hash>) -> MerkleTree {
        todo!()
    }

    /// Check if the given data produces the given root hash
    fn verify(input: &[Data], root: Hash) -> bool {
        todo!()
    } 

    /// Returns the root hash of the MerkleTree
    fn root_hash(&self) -> Hash {
        todo!()
    }

    fn get_merkle_proof_by_leaf_index(index: usize) -> bool {
        todo!()
    }

    fn get_merkle_proof_by_data(data: Hash) {
        todo!()
    }

}
