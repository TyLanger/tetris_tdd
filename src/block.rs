pub struct Block {
    letter: String,
}

impl Block {
    pub fn new(letter: &str) -> Self {
        Block {
            letter: letter.to_string(),
        }
    }
}
