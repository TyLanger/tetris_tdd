use crate::block::Block;

pub struct Board {
    width: usize,
    height: usize,
    blocks: Vec<Option<Block>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut blocks = Vec::with_capacity(width * height);
        for _ in 0..height {
            for _ in 0..width {
                blocks.push(None);
            }
        }
        Board {
            width,
            height,
            blocks,
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_string();
        for row in 0..self.height {
            let row = self.get_row(row);
            let mut row_string = "".to_string();
            for block in row {
                let letter = if let Some(b) = block {
                    b.letter.clone()
                } else {
                    ".".to_string()
                };
                row_string = format!("{}{}", row_string, letter);
            }
            s = format!("{}{}\n", s, row_string);
        }
        s.trim_end().to_string()
        // self.string.clone()
    }

    pub fn drop(&mut self, block: Block) {
        if let Some(elem) = self.blocks.get_mut(1) {
            *elem = Some(block);
        }
    }

    fn get_row(&self, from_top: usize) -> &[Option<Block>] {
        let offset = from_top;
        let start = offset * self.width;
        let end = start + self.width;
        &self.blocks[start..end]
    }

    pub fn tick(&mut self) {
        // swap top row with middle row
        self.blocks.swap(0, 3);
        self.blocks.swap(1, 4);
        self.blocks.swap(2, 5);

        // self.blocks.insert(index, element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;

    #[test]
    fn test_empty_rows() {
        let board = Board::new(3, 3);
        assert_eq!([None, None, None], board.get_row(0));
        assert_eq!([None, None, None], board.get_row(1));
        assert_eq!([None, None, None], board.get_row(2));
    }

    #[test]
    fn top_row_has_block_rest_empty() {
        let mut board = Board::new(3, 3);
        board.drop(Block::new("X"));

        assert_eq!(
            [
                None,
                Some(Block {
                    letter: "X".to_string()
                }),
                None
            ],
            board.get_row(0)
        );
        assert_eq!([None, None, None], board.get_row(1));
        assert_eq!([None, None, None], board.get_row(2));
    }
}
