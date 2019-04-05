use std::io;
use std::fmt::Error;
mod weapon;
use weapon::*;
mod result;
use result::*;

fn main() {
    loop {
        println!("Rock(r), Paper(p), Scissors(s)");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).unwrap();

        let weapon: Weapon = get_user_weapon(choice.to_lowercase().trim());
        let bot_weapon: Weapon = get_bot_weapon();


        play_round(&weapon, &bot_weapon);

    }


}

fn play_round(user_weapon: &Weapon, bot_weapon: &Weapon) {
    fn handle_rock(bot_weapon: &Weapon) -> ResultType {
        match bot_weapon {
            Weapon::ROCK => ResultType::TIE,
            Weapon::PAPER => ResultType::LOSS,
            Weapon::SCISSORS => ResultType::WIN,
        }
    }
    fn handle_paper(bot_weapon: &Weapon) -> ResultType {
        match bot_weapon {
            Weapon::ROCK => ResultType::WIN,
            Weapon::PAPER => ResultType::TIE,
            Weapon::SCISSORS => ResultType::LOSS,
        }
    }
    fn handle_scissors(bot_weapon: &Weapon) -> ResultType {
        match bot_weapon {
            Weapon::ROCK => ResultType::LOSS,
            Weapon::PAPER => ResultType::WIN,
            Weapon::SCISSORS => ResultType::TIE,
        }
    }
    let result = match user_weapon {
        Weapon::ROCK => handle_rock(bot_weapon),
        Weapon::PAPER => handle_paper(bot_weapon),
        Weapon::SCISSORS => handle_scissors(bot_weapon),
    };

    match result {
        ResultType::WIN => println!("You win! You chose: {:?} and the bot chose {:?}", user_weapon, bot_weapon),
        ResultType::LOSS => println!("You lose! You chose: {:?} and the bot chose {:?}", user_weapon, bot_weapon),
        ResultType::TIE => println!("You tie! You chose: {:?} and the bot chose {:?}", user_weapon, bot_weapon),
    }

}