use std::io::stdin;

fn main() {
    let mut player_one = String::new();
    let mut player_two = String::new();
    println!("Enter Player 1");
    stdin().read_line(&mut player_one).expect("a string");
    println!("Enter Player 2");
    stdin().read_line(&mut player_two).expect("a string");
    println!("{} vs {}", player_one.trim(), player_two.trim());
}
