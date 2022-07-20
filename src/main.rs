use std::io::stdin;

fn main() {
    let mut player_one = String::new();
    let mut player_two = String::new();

    println!("Enter Player 1");
    stdin().read_line(&mut player_one).expect("a string");
    println!("Enter Player 2");
    stdin().read_line(&mut player_two).expect("a string");

    let player1 = Player {
        name: String::from(player_one.trim()),
        sign: 'x'
    };
    let player2 = Player {
        name: String::from(player_two.trim()),
        sign: 'o'
    };

    print_players(&player1, &player2);

    let board = ['-'; 9];
    print_board(board);
}

struct Player {
    name: String,
    sign: char,
}

fn print_players(player1: &Player, player2: &Player) {
    println!("{} ({}) vs {} ({})" ,player1.name, player1.sign, player2.name, player2.sign)
}

fn print_board(board: [char; 9]) {
    println!("{} {} {}\n{} {} {}\n{} {} {}", board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]);
}