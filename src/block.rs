#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    pub letter: String,
}

impl Block {
    pub fn new(letter: &str) -> Self {
        Block {
            letter: letter.to_string(),
        }
    }
}
