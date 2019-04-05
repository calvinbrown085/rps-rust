use rand::Rng;
#[derive(Debug)]
pub enum Weapon {
    ROCK,
    PAPER,
    SCISSORS
}

pub fn get_user_weapon(player_choice: &str) -> Weapon {
    match player_choice {
        "rock" => Weapon::ROCK,
        "r" => Weapon::ROCK,
        "paper" => Weapon::PAPER,
        "p" => Weapon::PAPER,
        "scissors" => Weapon::SCISSORS,
        "s" => Weapon::SCISSORS,
        _ => panic!("Error"),
    }
}

pub fn get_bot_weapon() -> Weapon {
    let secret_number = rand::thread_rng().gen_range(1, 4);
    match secret_number {
        1 => Weapon::ROCK,
        2 => Weapon::PAPER,
        3 => Weapon::SCISSORS,
        _ => panic!("Should never get here!"),
    }
}