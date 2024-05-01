pub mod game {
    use std::io;

    pub const HAT: char = '^';
    pub const HOLE: char = 'O';
    pub const FIELD_CHAR: char = '░';
    pub const PATH_CHAR: char = '*';

    #[derive(Debug)]
    struct Game {
        board: Vec<Vec<char>>,
        index_for_y: usize,
        index_for_x: usize,
        state: bool,
    }

    impl Game {
        fn new(board: Vec<Vec<char>>) -> Self {
            let (index_for_y, index_for_x) = Game::find_starting_position(&board);

            Game {
                board,
                index_for_y,
                index_for_x,
                state: true,
            }
        }

        fn check_state(&mut self) {
            match self.board[self.index_for_y][self.index_for_x] {
                HAT => {
                    self.state = false;
                    println!("You won!");
                }
                HOLE => {
                    self.state = false;
                    println!("You lost!");
                }
                _ => {
                    println!("Find your hat! Use: W(UP), S(DOWN), D(RIGHT), A(LEFT)\n");
                }
            }
        }

        fn make_move(&mut self, user_input: &str) {
            print!("\x1B[2J\x1B[1;1H");
            match user_input.to_lowercase().as_str() {
                "w" => {
                    if self.index_for_y == 0 {
                        println!("The move is out of the boundaries!\n");
                        return;
                    }
                    self.index_for_y -= 1;
                }
                "s" => {
                    if self.index_for_y == (self.board.len() - 1) {
                        println!("The move is out of the boundaries!\n");
                        return;
                    }
                    self.index_for_y += 1;
                }
                "d" => {
                    if self.index_for_x == (self.board[0].len() - 1) {
                        println!("The move is out of the boundaries!\n");
                        return;
                    }
                    self.index_for_x += 1;
                }
                "a" => {
                    if self.index_for_x == 0 {
                        println!("The move is out of the boundaries!\n");
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
            println!("{:=<1$}", "", self.board[0].len());
            for idx in 0..self.board.len() {
                let mut part = String::new();
                for item in &self.board[idx] {
                    part.push_str(&item.to_string());
                }
                println!("{part}");
            }
            println!("{:=<1$}", "", self.board[0].len());
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
        let mut field = Game::new(board);
        println!("Find your hat! Use: W(UP), S(DOWN), D(RIGHT), A(LEFT)\n");
        while field.state {
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
