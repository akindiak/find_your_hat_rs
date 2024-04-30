pub mod game {
    use std::io;

    pub const HAT: char = '^';
    pub const HOLE: char = 'O';
    pub const FIELD_CHAR: char = '░';
    pub const PATH_CHAR: char = '*';

    #[derive(Debug)]
    pub struct Field {
        pub board: Vec<Vec<char>>,
        pub index_for_y: usize,
        pub index_for_x: usize,
        pub state: bool,
    }

    impl Field {
        pub fn new(board: Vec<Vec<char>>) -> Self {
            let (index_for_y, index_for_x) = Field::find_starting_position(&board);
            println!("Y:{}, X:{}", index_for_y, index_for_y);

            Field {
                board,
                index_for_y,
                index_for_x,
                state: true,
            }
        }

        fn check_state(&mut self) {
            match self.board[self.index_for_y][self.index_for_x] {
                FIELD_CHAR => {}
                PATH_CHAR => {}
                HAT => {
                    self.state = false;
                    println!("You won!");
                }
                HOLE => {
                    self.state = false;
                    println!("You lost!");
                }
                _ => {
                    self.state = false;
                    println!("You are out of the board!")
                }
            }
        }

        fn make_move(&mut self, user_input: &str) {
            match user_input.to_lowercase().as_str() {
                "w" => {
                    if self.index_for_y == 0 {
                        return;
                    }
                    self.index_for_y -= 1;
                }
                "s" => {
                    if self.index_for_y == (self.board.len() - 1) {
                        return;
                    }
                    self.index_for_y += 1;
                }
                "d" => {
                    if self.index_for_x == (self.board[0].len() - 1) {
                        return;
                    }
                    self.index_for_x += 1;
                }
                "a" => {
                    if self.index_for_x == 0 {
                        return;
                    }
                    self.index_for_x -= 1;
                }
                _ => {
                    println!("Ivalid input! Use: W(UP), S(DOWN), D(RIGHT), A(LEFT)");
                    return;
                }
            }
            self.check_state();
            self.board[self.index_for_y][self.index_for_x] = PATH_CHAR;
        }

        fn print_board(&self) {
            for idx in 0..self.board.len() {
                let mut part = String::new();
                for item in &self.board[idx] {
                    part.push_str(&item.to_string());
                }
                println!("{part}");
            }
        }

        fn find_starting_position(board: &Vec<Vec<char>>) -> (usize, usize) {
            for (idx, row) in board.iter().enumerate() {
                if let Some(position) = row.iter().position(|&el| el == PATH_CHAR) {
                    return (idx, position);
                } else {
                    continue;
                }
            }
            panic!("Could not find starting position!");
        }
    }

    pub fn run_game(board: Vec<Vec<char>>) {
        let mut field = Field::new(board);
        while field.state {
            print!("\x1B[2J\x1B[1;1H");
            println!("Find your hat! Use: W(UP), S(DOWN), D(RIGHT), A(LEFT)\n");
            field.print_board();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "stop" => field.state = false,
                _ => field.make_move(&input.trim()),
            }
        }
        println!("Thank you for the game!");
    }
}
