#[cfg(test)]
pub mod tests {
    #[test]
    fn test_success() {
        let my_hello = "Hello world!";
        assert_eq!(my_hello, "Hello world!");
    }
}
