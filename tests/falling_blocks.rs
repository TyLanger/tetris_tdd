use tetris_tdd::block::Block;
use tetris_tdd::board::{Board, BoardError};

#[test]
fn board_starts_empty() {
    let board = Board::new(3, 3);
    assert_eq!("...\n...\n...", board.to_string());
}

#[test]
fn block_drops_from_top_middle() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    assert_eq!(".X.\n...\n...", board.to_string())
}

#[test]
fn block_moves_down_one_row_per_tick() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    board.tick();
    assert_eq!("...\n.X.\n...", board.to_string())
}

#[test]
fn at_most_one_block_falling_at_a_time() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    let before = board.to_string();

    let err = board.drop(Block::new("Y"));
    assert_eq!(BoardError::AlreadyFalling, err.unwrap_err());
    let after = board.to_string();

    assert_eq!(before, after);
}
