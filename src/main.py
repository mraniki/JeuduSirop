import random

def draw_sirop_card():
    """Draw a random Sirop card."""
    sirop_cards = ["Sirop de 8", "Sirop de 14", "Sirop de 21"]
    return random.choice(sirop_cards)

def move_player(position, spaces):
    """Move the player on the board."""
    new_position = position + spaces
    return new_position

def play_sirop_game():
    """Main function to play the Sirop game."""
    target_numbers = [8, 14, 21]
    player_position = 1

    for target in target_numbers:
        while player_position < target:
            input("Press Enter to draw a Sirop card.")
            sirop_card = draw_sirop_card()
            print(f"You drew: {sirop_card}")

            # Extract the number from the Sirop card
            sirop_number = int(sirop_card.split()[-1])

            # Move the player
            player_position = move_player(player_position, sirop_number)
            print(f"Current position: {player_position}")

            if player_position == target:
                print(f"Congratulations! You reached {target}!")
            elif player_position > target:
                print("Oops! You went too far. Moving back.")
                player_position -= (player_position - target)

if __name__ == "__main__":
    print("Welcome to the Sirop game!")
    play_sirop_game()
