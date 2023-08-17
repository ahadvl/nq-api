pub mod token;

#[cfg(test)]
mod tests {
    use crate::token::HashBuilder;

    #[test]
    fn test_token_generator() {
        let token = HashBuilder::new(&String::from("Hello World").into_bytes())
            .generate()
            .get_result();
    }
}
