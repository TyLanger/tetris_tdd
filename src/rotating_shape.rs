use std::fmt::Display;

pub struct RotatingShape {
    letters: Vec<String>,
}

impl RotatingShape {
    pub fn new(letters: &str) -> Self {
        let mut abc: Vec<String> = Vec::new();
        for letter in letters.split("").skip_while(|x| x.is_empty()) {
            abc.push(letter.to_string());
        }
        // abc.truncate(9);
        RotatingShape { letters: abc }
    }

    fn swap_four(&mut self, a: usize, b: usize, c: usize, d: usize) {
        self.letters.swap(a, b);
        self.letters.swap(a, c);
        self.letters.swap(a, d);
    }

    pub fn rotate_right(&mut self) {
        let big = self.letters.len() > 10;
        let width = if big { 5 } else { 3 };
        let height = if big { 5 } else { 3 };

        // corners
        self.swap_four(0, width - 1, width * height - 1, width * height - width);

        // mid
        let a = width / 2; // 1 or 2
        let b = ((a + 1) * height) - 1; // 5 or 14
        let c = width * height - (a + 1); // 7 or 22
        let d = a * height; // 3 or 10
        self.swap_four(a, b, c, d);

        if big {
            // doing it custom for a 5x5 is just easier than being general
            // as per the tetris specs, shouldn't need bigger
            self.swap_four(1, 9, 23, 15);
            self.swap_four(3, 19, 21, 5);
            self.swap_four(6, 8, 18, 16);
            self.swap_four(7, 13, 17, 11);
        }
    }

    pub fn rotate_left(&mut self) {
        let big = self.letters.len() > 10;
        let width = if big { 5 } else { 3 };
        let height = if big { 5 } else { 3 };

        // corners
        // right is swap_four(a, b, c, d)
        // left is swap_four(a, d, c, b) swap b and d
        self.swap_four(0, width * height - width, width * height - 1, width - 1);

        // mid
        let a = width / 2; // 1 or 2
        let b = ((a + 1) * height) - 1; // 5 or 14
        let c = width * height - (a + 1); // 7 or 22
        let d = a * height; // 3 or 10
        self.swap_four(a, d, c, b);

        if big {
            self.swap_four(1, 15, 23, 9);
            self.swap_four(3, 5, 21, 19);
            self.swap_four(6, 16, 18, 8);
            self.swap_four(7, 11, 17, 13);
        }
    }
}

impl Display for RotatingShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.shape_letters)
        let mut string = "".to_string();
        for letter in self.letters.iter() {
            string = format!("{}{}", string, letter);
        }

        write!(f, "{}", string)
        // write!(f, "{:?}", self.letters)
    }
}
