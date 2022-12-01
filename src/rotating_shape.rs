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
        RotatingShape { letters: abc }
    }

    pub fn from_rotate_right(&self) -> Self {
        RotatingShape {
            letters: self.get_right_letters(),
        }
    }

    pub fn from_rotate_left(&self) -> Self {
        RotatingShape {
            letters: self.get_left_letters(),
        }
    }

    pub fn rotate_right(&mut self) {
        self.letters = self.get_right_letters();
    }

    pub fn rotate_left(&mut self) {
        self.letters = self.get_left_letters();
    }

    fn get_right_letters(&self) -> Vec<String> {
        let big = self.letters.len() > 10;
        let width = if big { 5 } else { 3 };
        let height = if big { 5 } else { 3 };

        let mut rotated_letters = vec!["".to_string(); width * height];

        let mut count = 0;
        for i in 0..width {
            let mut offset = width - (i + 1);
            for _j in 0..height {
                rotated_letters[offset] = self.letters[count].clone();
                count += 1;
                offset += width;
            }
        }
        rotated_letters
    }

    fn get_left_letters(&self) -> Vec<String> {
        let big = self.letters.len() > 10;
        let width = if big { 5 } else { 3 };
        let height = if big { 5 } else { 3 };

        let mut rotated_letters = vec!["".to_string(); width * height];

        let mut count = 0;
        for i in 0..width {
            // start at 9 or 25
            // imaginary row beneath
            let mut offset = width * height + i;
            for _j in 0..height {
                // sub before
                offset -= width;
                rotated_letters[offset] = self.letters[count].clone();
                count += 1;
                // if I sub after,
                // will go negative
                // but then the for loop ends and the neg value isn't used
                // this saves me from checking if it would be less than 0
            }
        }
        rotated_letters
    }
}

impl Display for RotatingShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = "".to_string();
        for letter in self.letters.iter() {
            string = format!("{}{}", string, letter);
        }

        write!(f, "{}", string)
    }
}
