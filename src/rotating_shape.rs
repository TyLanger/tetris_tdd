use std::fmt::Display;



pub struct RotatingShape {
    shape_letters: String,
}

impl RotatingShape {
    pub fn new(letters: &str) -> Self {
        RotatingShape { shape_letters: letters.to_string() }
    }
}

impl Display for RotatingShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.shape_letters)
    }
}