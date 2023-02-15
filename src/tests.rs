
#[cfg(test)]
mod tests {
    use crate::MerkleTree;

    #[test]
    fn create_two_levels_merkle_tree() {
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
}
