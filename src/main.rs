use std::io;
use std::fmt::Error;
mod weapon;
use weapon::*;

fn main() {
    loop {
        println!("Rock, Paper, or Scissors?");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).unwrap();

        let weapon: Weapon = get_user_weapon(choice.to_lowercase().trim());
        let bot_weapon: Weapon = get_bot_weapon();

        play_round(&weapon, &bot_weapon);
    }


}

fn play_round(user_weapon: &Weapon, bot_weapon: &Weapon) {
    fn handle_rock(bot_weapon: &Weapon) -> bool {
        match bot_weapon {
            Weapon::ROCK => true,
            Weapon::PAPER => false,
            Weapon::SCISSORS => true,
        }
    }
    fn handle_paper(bot_weapon: &Weapon) -> bool {
        match bot_weapon {
            Weapon::ROCK => true,
            Weapon::PAPER => true,
            Weapon::SCISSORS => false,
        }
    }
    fn handle_scissors(bot_weapon: &Weapon) -> bool {
        match bot_weapon {
            Weapon::ROCK => false,
            Weapon::PAPER => true,
            Weapon::SCISSORS => true,
        }
    }
    let result = match user_weapon {
        Weapon::ROCK => handle_rock(bot_weapon),
        Weapon::PAPER => handle_paper(bot_weapon),
        Weapon::SCISSORS => handle_scissors(bot_weapon),
    };

    match result {
        true => println!("You win! You chose: {:?} and the bot chose {:?}", user_weapon, bot_weapon),
        false => println!("You lose! You chose: {:?} and the bot chose {:?}", user_weapon, bot_weapon),
    }


}