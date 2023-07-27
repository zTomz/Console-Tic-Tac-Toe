pub struct GameField {
    a1: char,
    a2: char,
    a3: char,
    b1: char,
    b2: char,
    b3: char,
    c1: char,
    c2: char,
    c3: char,
}

impl GameField {
    pub fn new() -> GameField {
        GameField {
            a1: ' ',
            a2: ' ',
            a3: ' ',
            b1: ' ',
            b2: ' ',
            b3: ' ',
            c1: ' ',
            c2: ' ',
            c3: ' ',
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} | {} | {}\n{} | {} | {}\n{} | {} | {}",
            self.a1, self.a2, self.a3, self.b1, self.b2, self.b3, self.c1, self.c2, self.c3
        )
    }

    pub fn make_move(&mut self, number: u32, player: char) {
        match number {
            1 => self.a1 = player,
            2 => self.a2 = player,
            3 => self.a3 = player,
            4 => self.b1 = player,
            5 => self.b2 = player,
            6 => self.b3 = player,
            7 => self.c1 = player,
            8 => self.c2 = player,
            9 => self.c3 = player,
            _ => (),
        }
    }

    pub fn check_move(&self, number: u32) -> bool {
        match number {
            1 => self.a1 == ' ',
            2 => self.a2 == ' ',
            3 => self.a3 == ' ',
            4 => self.b1 == ' ',
            5 => self.b2 == ' ',
            6 => self.b3 == ' ',
            7 => self.c1 == ' ',
            8 => self.c2 == ' ',
            9 => self.c3 == ' ',
            _ => false,
        }
    }

    pub fn check_win(&self) -> char {
        if self.a1 == self.a2 && self.a2 == self.a3 {
            self.a1
        } else if self.b1 == self.b2 && self.b2 == self.b3 {
            self.b1
        } else if self.c1 == self.c2 && self.c2 == self.c3 {
            self.c1
        } else if self.a1 == self.b1 && self.b1 == self.c1 {
            self.a1
        } else if self.a2 == self.b2 && self.b2 == self.c2 {
            self.a2
        } else if self.a3 == self.b3 && self.b3 == self.c3 {
            self.a3
        } else if self.a1 == self.b2 && self.b2 == self.c3 {
            self.a1
        } else if self.a3 == self.b2 && self.b2 == self.c1 {
            self.a3
        } else {
            ' '
        }
    }
}
