use find_your_hat::game;

fn main() {
    let board = vec![vec!['*', '░', '░'],
                                     vec!['O', 'O', '░'],
                                     vec!['░', '░', '░'],
                                     vec!['░', 'O', 'O'],
                                     vec!['░', 'O', '░'],
                                     vec!['░', '░', '░'],
                                     vec!['░', 'O', '^']];
    game::run_game(board)
}
