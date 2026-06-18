pub fn fixture(path: &str) -> String {
    std::fs::read_to_string(path).expect("fixture should exist")
}
