use tetris_tdd::rotating_shape::RotatingShape;

#[test]
fn rotating_3x3_shape() {
    let shape = RotatingShape::new("ABCDEFGHI");

    assert_eq!("ABCDEFGHI".to_string(), shape.to_string());
}

#[test]
fn can_be_rotated_right() {
    // right / clockwise
    let mut shape = RotatingShape::new("ABCDEFGHI");

    shape.rotate_right();

    assert_eq!("GDAHEBIFC".to_string(), shape.to_string());
}

#[test]
fn can_be_rotated_left() {
    // left / counter clockwise
    let mut shape = RotatingShape::new("ABCDEFGHI");

    shape.rotate_left();

    assert_eq!("CFIBEHADG".to_string(), shape.to_string());
}

#[test]
fn rotating_5x5_shape() {
    let shape = RotatingShape::new("ABCDEFGHIJKLMNOPQRSTUVWXY");

    assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXY".to_string(), shape.to_string());
}

#[test]
fn big_shape_can_be_rotated_right() {
    let mut shape = RotatingShape::new("ABCDEFGHIJKLMNOPQRSTUVWXY");

    shape.rotate_right();

    assert_eq!("UPKFAVQLGBWRMHCXSNIDYTOJE".to_string(), shape.to_string());
}

#[test]
fn big_shape_can_be_rotated_left() {
    let mut shape = RotatingShape::new("ABCDEFGHIJKLMNOPQRSTUVWXY");

    shape.rotate_left();

    assert_eq!("EJOTYDINSXCHMRWBGLQVAFKPU".to_string(), shape.to_string());
}

#[test]
fn rotate_right_immutable() {
    let shape = RotatingShape::new("ABCDEFGHIJKLMNOPQRSTUVWXY");

    // return a new shape
    // don't edit the current one
    assert_eq!(
        "UPKFAVQLGBWRMHCXSNIDYTOJE".to_string(),
        shape.from_rotate_right().to_string()
    );
    assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXY".to_string(), shape.to_string());
}

#[test]
fn rotate_left_immutable() {
    let shape = RotatingShape::new("ABCDEFGHIJKLMNOPQRSTUVWXY");

    // return a new shape
    // don't edit the current one
    assert_eq!(
        "EJOTYDINSXCHMRWBGLQVAFKPU".to_string(),
        shape.from_rotate_left().to_string()
    );
    assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXY".to_string(), shape.to_string());
}
