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

    let mut board = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    
    let mut round = 1;
    let result = loop {
        if round > 9 { break '-' };

        println!("----------");
        println!("ROUND {}", round);
        print_board(board);

        let Player {name: player_name, sign: player_sign} = if round%2 == 1 {&player1} else {&player2};
        println!("{}'s turn ({})", player_name, player_sign);

        let mut pos_input = String::new();
        println!("Enter position");
        stdin().read_line(&mut pos_input).expect("a string");
        let pos: usize = match pos_input.trim().parse() {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        if !insert_sign(&mut board, *player_sign, pos) {
            println!("invalid position! please try again");
            continue;
        }

        if check_winner(board) == 'x' {break 'x'}
        else if check_winner(board) == 'o' {break 'o'}
        round += 1;
    };

    println!("----------");
    println!("GAME OVER");
    print_board(board);

    if result != '-' {
        println!("The winner is {}!", result);
    } else {
        println!("It's a tie!")
    }
}

struct Player {
    name: String,
    sign: char,
}

fn print_players(player1: &Player, player2: &Player) {
    println!("{} ({}) vs {} ({})" ,player1.name, player1.sign, player2.name, player2.sign);
}

fn print_board(board: [char; 9]) {
    println!("----------\n{} {} {}\n{} {} {}\n{} {} {}\n----------", 
    board[0], board[1], board[2],
    board[3], board[4], board[5],
    board[6], board[7], board[8]);
}

fn insert_sign(board: &mut [char; 9], sign: char, pos: usize) -> bool {
    if pos < 1 || pos > 9 { return false }

    let pos_index = pos - 1;
    if board[pos_index] == 'x' || board[pos_index] == 'o' { return false }

    board[pos_index] = sign;
    true
}

fn check_winner(board: [char; 9]) -> char {
    //horizontals
    if check_three(board, 0, 1, 2) { board[0] }
    else if check_three(board, 3, 4, 5) { board[3] }
    else if check_three(board, 6, 7, 8) { board[6] }
    //verticals
    else if check_three(board, 0, 3, 6) { board[0] }
    else if check_three(board, 1, 4, 7) { board[1] }
    else if check_three(board, 2, 5, 8) { board[2] }
    //diagonals
    else if check_three(board, 0, 4, 8) { board[0] }
    else if check_three(board, 2, 4, 6) { board[2] }
    //no winner
    else {'-'}
}

fn check_three(board: [char; 9], i1: usize, i2: usize, i3: usize) -> bool {
    if board[i1] == board[i2] && board[i1] == board[i3] { true } else { false }
}