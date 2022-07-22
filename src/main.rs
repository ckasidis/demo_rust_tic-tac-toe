use std::io::stdin;
use rand::Rng;

fn main() {    
    let mut game = Game::new();
    game.print_players();
    
    let result = loop {
        if game.round > 9 { break '-' };

        println!("----------");
        println!("ROUND {}", game.round);
        game.print_board();

        let player_name = if game.round%2 == 1 { &game.player_x } else { &game.player_o };
        let player_sign = if game.round%2 == 1 { 'x' } else { 'o' };
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

        if !game.insert_sign(player_sign, pos) {
            println!("invalid position! please try again");
            continue;
        }

        if game.check_winner() == 'x' {break 'x'}
        else if game.check_winner() == 'o' {break 'o'}

        game.round += 1;
    };

    println!("----------");
    println!("GAME OVER");
    game.print_board();

    if result != '-' {
        println!("The winner is {}!", result);
    } else {
        println!("It's a tie!")
    }
}

struct Game {
    board: [char; 9],
    player_x: String,
    player_o: String,
    round: u8,
}

impl Game {
    fn new() -> Game {
        let mut player_one = String::new();
        let mut player_two = String::new();
    
        println!("Enter Player 1");
        stdin().read_line(&mut player_one).expect("a string");
        println!("Enter Player 2");
        stdin().read_line(&mut player_two).expect("a string");

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..2);
    
        Game {
            board: ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
            player_x: String::from(if random_number == 0 {player_one.trim()} else {player_two.trim()}),
            player_o: String::from(if random_number == 0 {player_two.trim()} else {player_one.trim()}),
            round: 1,
        }
    }

    fn print_players(&self) {
        println!("{} ({}) vs {} ({})", self.player_x, 'x', self.player_o, 'o');
    }

    fn print_board(&self) {
        let board = self.board;
        println!("----------\n{} {} {}\n{} {} {}\n{} {} {}\n----------", 
        board[0], board[1], board[2],
        board[3], board[4], board[5],
        board[6], board[7], board[8]);
    }

    fn insert_sign(&mut self, sign: char, pos: usize) -> bool {
        if pos < 1 || pos > 9 { return false }
    
        let pos_index = pos - 1;
        if self.board[pos_index] == 'x' || self.board[pos_index] == 'o' { return false }
    
        self.board[pos_index] = sign;
        true
    }

    fn check_winner(&self) -> char {
        //horizontals
        if self.check_three(0, 1, 2) { self.board[0] }
        else if self.check_three(3, 4, 5) { self.board[3] }
        else if self.check_three(6, 7, 8) { self.board[6] }
        //verticals
        else if self.check_three(0, 3, 6) { self.board[0] }
        else if self.check_three(1, 4, 7) { self.board[1] }
        else if self.check_three(2, 5, 8) { self.board[2] }
        //diagonals
        else if self.check_three(0, 4, 8) { self.board[0] }
        else if self.check_three(2, 4, 6) { self.board[2] }
        //no winner
        else {'-'}
    }

    fn check_three(&self, i1: usize, i2: usize, i3: usize) -> bool {
        if self.board[i1] == self.board[i2] && self.board[i1] == self.board[i3] { true } else { false }
    }
}




