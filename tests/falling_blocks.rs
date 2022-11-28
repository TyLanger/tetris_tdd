use tetris_tdd::board::Board;

#[test]
fn board_starts_empty() {
    let board = tetris_tdd::board::Board::new(3, 3);
    assert_eq!("...\n...\n...", board.to_string());
}

#[test]
fn block_drops_from_middle() {
    let mut board = Board::new(3, 3);
    board.drop(tetris_tdd::block::Block::new("X"));
    assert_eq!(".X.\n...\n...", board.to_string())
}
