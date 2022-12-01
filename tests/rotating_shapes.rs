use tetris_tdd::rotating_shape::RotatingShape;



#[test]
fn rotating_3x3_shape() {
    let shape = RotatingShape::new(
        "ABCDEFGHI"
    );

    assert_eq!("ABCDEFGHI".to_string(), shape.to_string());
}