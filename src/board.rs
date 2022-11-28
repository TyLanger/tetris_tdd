use crate::block::Block;

pub struct Board {
    width: u32,
    height: u32,

    string: String,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Board {
            width,
            height,
            string: "...\n...\n...".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.string.clone()
    }

    pub fn drop(&mut self, block: Block) {
        self.string = ".X.\n...\n...".to_string();
    }
}
