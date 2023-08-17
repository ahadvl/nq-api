pub mod token;

#[cfg(test)]
mod tests {
    use crate::token::HashBuilder;

    #[test]
    fn test_hash_builder() {
        let token = HashBuilder::new()
            .set_source(&String::from("Hello World").into_bytes())
            .generate()
            .get_result();

        assert_eq!(
            token.unwrap(),
            "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e".to_string()
        );
    }
}
