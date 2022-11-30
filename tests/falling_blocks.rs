use tetris_tdd::block::Block;
use tetris_tdd::board::Board;

#[test]
fn board_starts_empty() {
    let board = Board::new(3, 3);
    assert_eq!("...\n...\n...", board.to_string());
}

#[test]
fn block_drops_from_top_middle() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X"));
    assert_eq!(".X.\n...\n...", board.to_string())
}

#[test]
fn block_moves_down_one_row_per_tick() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X"));
    board.tick();
    assert_eq!("...\n.X.\n...", board.to_string())
}
