pub fn format_todo_summary(todo_text: &str) -> String {
    format!("{}{}", &todo_text[0..12], "...")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
