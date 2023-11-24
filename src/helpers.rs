pub mod helper_functions {
    pub fn convert_to_strings<T: ToString>(input: Vec<T>) -> Vec<String> {
        input.into_iter().map(|item| item.to_string()).collect()
    }
}
