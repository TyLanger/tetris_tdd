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

#[test]
fn block_reaches_the_bottom() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    board.tick();
    board.tick();

    assert_eq!("...\n...\n.X.", board.to_string())
}

#[test]
fn block_still_moving_at_bottom() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    board.tick();
    board.tick();

    // the player should still be able to move the block
    assert_eq!(true, board.has_falling());
}

#[test]
fn block_stops_moving_at_bottom() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    board.tick();
    board.tick();
    board.tick();

    assert_eq!("...\n...\n.X.", board.to_string());
    assert_eq!(false, board.has_falling());
}

#[test]
fn can_drop_second_block() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    board.tick();
    board.tick();
    board.tick();

    assert_eq!("...\n...\n.X.", board.to_string());
    assert_eq!(false, board.has_falling());

    board.drop(Block::new("Y")).unwrap();
    assert_eq!(".Y.\n...\n.X.", board.to_string());
    assert_eq!(true, board.has_falling());
}

#[test]
fn is_still_moving_on_row_above_other_block() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    board.tick();
    board.tick();
    board.tick();

    assert_eq!("...\n...\n.X.", board.to_string());
    assert_eq!(false, board.has_falling());

    board.drop(Block::new("Y")).unwrap();
    board.tick();

    assert_eq!("...\n.Y.\n.X.", board.to_string());
    assert_eq!(true, board.has_falling());
}

#[test]
fn new_block_stops_on_top_of_old_block() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    board.tick();
    board.tick();
    board.tick();
    board.drop(Block::new("Y")).unwrap();
    board.tick();
    board.tick();

    assert_eq!("...\n.Y.\n.X.", board.to_string());
    assert_eq!(false, board.has_falling());
}

#[test]
fn third_block_stacks() {
    let mut board = Board::new(3, 3);
    board.drop(Block::new("X")).unwrap();
    board.tick();
    board.tick();
    board.tick();
    board.drop(Block::new("Y")).unwrap();
    board.tick();
    board.tick();
    board.drop(Block::new("Z")).unwrap();
    board.tick();

    assert_eq!(".Z.\n.Y.\n.X.", board.to_string());
    assert_eq!(false, board.has_falling());
}
