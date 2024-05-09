use find_your_hat::game;
use std::io;

fn main() {
    let mut input = String::new();
    println!("Choose complexity 0-100%");
    io::stdin().read_line(&mut input).unwrap();
    let complexity_percentage: u32 = input.trim().parse().unwrap();

    let _test_board = vec![vec!['*', '░', '░'],
                                           vec!['O', 'O', '░'],
                                           vec!['░', '░', '░'],
                                           vec!['░', 'O', 'O'],
                                           vec!['░', 'O', '░'],
                                           vec!['░', '░', '░'],
                                           vec!['░', 'O', '^']];
    
    let new_board = game::generate_board(10, 40, Some(complexity_percentage));
    game::run_game(new_board);
}
