use std::io::stdin;

fn main() {
    let mut player_one = String::new();
    let mut player_two = String::new();
    println!("Enter Player 1");
    stdin().read_line(&mut player_one).expect("a string");
    println!("Enter Player 2");
    stdin().read_line(&mut player_two).expect("a string");
    println!("{} vs {}", player_one.trim(), player_two.trim());

    let board = ['-'; 9];
    print_board(board);
}

 fn print_board(board: [char; 9]) {
    println!("{} {} {}\n{} {} {}\n{} {} {}", board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]);
}