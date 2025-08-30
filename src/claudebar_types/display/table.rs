pub struct HeaderInfo {
    pub label: String,
    pub width: usize,
}

impl HeaderInfo {
    pub fn new(label: &str, width: usize) -> Self {
        Self {
            label: label.to_string(),
            width,
        }
    }
}