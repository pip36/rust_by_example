pub fn get_todo() -> String {
    String::from("I'm a note that's potentially very long and uninteresting.")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
