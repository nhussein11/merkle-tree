#[cfg(test)]
mod tests {

    use crate::{Error, MerkleTree};

    #[test]
    fn create_two_levels_merkle_tree() {
        let items = vec![
            String::from("a").into_bytes(),
            String::from("b").into_bytes(),
            String::from("c").into_bytes(),
        ];

        let merkle_tree = MerkleTree::construct_by_input(&items);
        // The Merkle tree should have 4 leaves (3 original + 1 pad), 2 internal nodes and 1 root node (4 + 2 + 1 = 7)
        assert_eq!(merkle_tree.nodes.len(), 7);
    }

    #[test]
    fn create_three_levels_merkle_tree() {
        let items = vec![
            String::from("a").into_bytes(),
            String::from("b").into_bytes(),
            String::from("c").into_bytes(),
            String::from("d").into_bytes(),
            String::from("e").into_bytes(),
        ];

        let merkle_tree = MerkleTree::construct_by_input(&items);
        
        // The Merkle tree should have 8 leaves (5 original + 3 pad), 6 (4+2) internal nodes and 1 root node (8 + 6 + 1 = 13)
        assert_eq!(merkle_tree.nodes.len(), 15);
    }

    #[test]
    fn verify_merkle_tree() {
        let items = vec![
            String::from("a").into_bytes(),
            String::from("b").into_bytes(),
            String::from("c").into_bytes(),
            String::from("d").into_bytes(),
            String::from("e").into_bytes(),
        ];

        let merkle_tree = MerkleTree::construct_by_input(&items);

        assert!(MerkleTree::verify(&items, merkle_tree.root_hash()));
    }

    //#[test]
    //fn verify_merkle_tree_should_fail() {
    //    let items = vec![
    //        String::from("a").into_bytes(),
    //        String::from("b").into_bytes(),
    //        String::from("c").into_bytes(),
    //        String::from("d").into_bytes(),
    //        String::from("e").into_bytes(),
    //    ];

    //    let merkle_tree = MerkleTree::construct_by_input(&items);

    //    let mut items = items.clone();
    //    items.push(String::from("f").into_bytes());

    //    assert!(!MerkleTree::verify(&items, merkle_tree.root_hash()));
    //}

    //#[test]
    //fn get_merkle_tree_proof_by_leave_index() {
    //    let items = vec![
    //        String::from("a").into_bytes(),
    //        String::from("b").into_bytes(),
    //        String::from("c").into_bytes(),
    //        String::from("d").into_bytes(),
    //        String::from("e").into_bytes(),
    //    ];

    //    let merkle_tree = MerkleTree::construct_by_input(&items);

    //    let proof = merkle_tree.get_merkle_proof_by_leaf_index(0).unwrap();

    //    assert_eq!(proof.len(), 3);
    //}

    //#[test]
    //fn get_merkle_tree_proof_by_leave_index_should_fail() {
    //    let items = vec![
    //        String::from("a").into_bytes(),
    //        String::from("b").into_bytes(),
    //        String::from("c").into_bytes(),
    //        String::from("d").into_bytes(),
    //        String::from("e").into_bytes(),
    //    ];

    //    let merkle_tree = MerkleTree::construct_by_input(&items);

    //    let proof = merkle_tree.get_merkle_proof_by_leaf_index(15);

    //    assert_eq!(proof, Err(Error::InvalidLeafIndex));
    //}

    //#[test]
    //fn get_merkle_tree_proof_by_data() {
    //    let items = vec![
    //        String::from("a").into_bytes(),
    //        String::from("b").into_bytes(),
    //        String::from("c").into_bytes(),
    //        String::from("d").into_bytes(),
    //        String::from("e").into_bytes(),
    //    ];

    //    let merkle_tree = MerkleTree::construct_by_input(&items);

    //    let proof = merkle_tree
    //        .get_merkle_proof_by_data(String::from("a").into_bytes())
    //        .unwrap();

    //    assert_eq!(proof.len(), 3);
    //}

    //#[test]
    //fn get_merkle_tree_proof_by_data_should_fail() {
    //    let items = vec![
    //        String::from("a").into_bytes(),
    //        String::from("b").into_bytes(),
    //        String::from("c").into_bytes(),
    //        String::from("d").into_bytes(),
    //        String::from("e").into_bytes(),
    //    ];

    //    let merkle_tree = MerkleTree::construct_by_input(&items);

    //    let proof = merkle_tree.get_merkle_proof_by_data(String::from("f").into_bytes());

    //    assert_eq!(proof, Err(Error::InvalidData));
    //}
}
