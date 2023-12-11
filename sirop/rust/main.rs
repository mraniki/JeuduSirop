use std::io;

fn draw_sirop_card() -> &'static str {
    // Draw a random Sirop card
    let sirop_cards = ["Sirop de 8", "Sirop de 14", "Sirop de 21"];
    let index = rand::random::<usize>() % sirop_cards.len();
    sirop_cards[index]
}

fn move_player(position: usize, spaces: usize) -> usize {
    // Move the player on the board
    position + spaces
}

fn play_sirop_game() {
    // Main function to play the Sirop game
    let target_numbers = [8, 14, 21];
    let mut player_position = 1;

    for &target in target_numbers.iter() {
        while player_position < target {
            let mut input = String::new();
            println!("Press Enter to draw a Sirop card.");
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let sirop_card = draw_sirop_card();
            println!("You drew: {}", sirop_card);

            // Extract the number from the Sirop card
            let sirop_number: usize = sirop_card.split_whitespace().last().unwrap().parse().unwrap();

            // Move the player
            player_position = move_player(player_position, sirop_number);
            println!("Current position: {}", player_position);

            if player_position == target {
                println!("Congratulations! You reached {}!", target);
            } else if player_position > target {
                println!("Oops! You went too far. Moving back.");
                player_position -= player_position - target;
            }
        }
    }
}

fn main() {
    println!("Welcome to the Sirop game!");
    play_sirop_game();
}