pub fn greet(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_has_name() {
        let result = greet("Akshat");
        assert!(
            result.contains("Akshat"),
            "Greeting does not contain the name as expected. Value was `{result}`."
        )
    }
}
