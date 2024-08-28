#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let hlw = "Hello World".to_string();
        let bytes = hlw.as_bytes();
        let len = bytes.len();

        println!("{len}")
    }
}