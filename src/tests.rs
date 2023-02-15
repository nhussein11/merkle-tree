
#[cfg(test)]
mod tests {
    use crate::MerkleTree;

    #[test]
    fn create_two_levels_merkle_tree(){
        let items = vec![
            String::from("a").into_bytes(),
            String::from("b").into_bytes(),
            String::from("c").into_bytes(),
        ];

        let merkle_tree = MerkleTree::construct_by_input(&items);

        assert_eq!(merkle_tree.levels, 2);
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

        assert_eq!(merkle_tree.levels, 3);
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

    #[test] 
    fn get_merkle_tree_proof_by_leave_index(){
        let items = vec![
            String::from("a").into_bytes(),
            String::from("b").into_bytes(),
            String::from("c").into_bytes(),
            String::from("d").into_bytes(),
            String::from("e").into_bytes(),
        ];

        let merkle_tree = MerkleTree::construct_by_input(&items);

        let proof = merkle_tree.get_merkle_proof_by_leaf_index(0).unwrap();

        assert_eq!(proof.len(), 3);
    }
}
