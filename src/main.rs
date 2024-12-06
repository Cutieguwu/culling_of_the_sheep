mod characters;
mod events;
mod utils;

use characters::{
    Character,
    People,
    SurvivalStatus
};
use events::{
    get_event, EventHandle, EventReturn, Events, Trial
};
use rand::{
    random,
    thread_rng
};
use utils::{
    fmt_args,
    ArgType,
    FlagType
};

//Number of game rounds.
const ROUNDS: u8 = 15;

fn main() {
    let cmd_args = dbg!(fmt_args());
    let mut characters = People::build_characters();
    let mut rng = thread_rng();

    // Set player default values based on gamemode.
    let mut player: Character = dbg!(Character::build_player(Gamemode::get(&cmd_args)));
    let mut end_status = SurvivalStatus::PlayerLived;
    
    // Game loop.
    for r in 0..=ROUNDS {
        let random_event: Events = random();

        match random_event {
            Events::Trial(trial) => match trial.handle(&rng, &mut player, &mut characters) {
                EventReturn::Survival(state) => match state {
                    SurvivalStatus::PlayerLived => (),
                    _ => {end_status = state}
                }
                _ => unreachable!()
            }
        }

        match end_status {
            SurvivalStatus::PlayerLived => (),
            _ => break
        }

        // If player's standing is too low, bring to court.
        if player.standing.clone().calculate() <= 0 {
            match Trial.handle(&rng, &mut player, &mut characters) {
                EventReturn::Survival(state) => match state {
                    SurvivalStatus::PlayerLived => (),
                    _ => {end_status = state}
                },
                _ => unreachable!() 
            };
        };

        if r == ROUNDS {
            end_status = SurvivalStatus::PlayerLived
        };

        if characters.len() == 0 {
            end_status = SurvivalStatus::PlayerMassacred
        };
    };

    match end_status {
        SurvivalStatus::PlayerDied => println!("You died. And for what?"),
        SurvivalStatus::PlayerLived => println!("You survived, but at what cost? How many died to save you?"),
        SurvivalStatus::PlayerMassacred => println!("You killed everyone. I ask, at what end will you yield. Care you not for the life of others?")
    }
}

#[derive(Debug)]
pub enum Gamemode {
    //Easy
    Abigail,
    //Normal
    MaryWarren,
    //Hard
    JohnProctor,
    //Just dead.
    GoodyOsborne
}

impl Gamemode {
    fn get(cmd_args: &Vec<ArgType>) -> Gamemode {
        if cmd_args.len() != 1 {
            match &cmd_args[1] {
                ArgType::Command(c) => match c.as_str() {
                    "abigail" => Gamemode::Abigail,
                    "john_proctor" => Gamemode::JohnProctor,
                    "goody_osborne" => Gamemode::GoodyOsborne,
                    _ => Gamemode::MaryWarren
                },
                ArgType::Flag(f) => match f {
                    FlagType::Long(l) => match l.as_str() {
                        "abigail" => Gamemode::Abigail,
                        "john-proctor" => Gamemode::JohnProctor,
                        "goody-osborne" => Gamemode::GoodyOsborne,
                        _ => Gamemode::MaryWarren
                    },
                    FlagType::Short(s) => match s.as_str() {
                        "a" => Gamemode::Abigail,
                        "j" => Gamemode::JohnProctor,
                        "o" => Gamemode::GoodyOsborne,
                        _ => Gamemode::MaryWarren
                    }
                },
                _ => Gamemode::MaryWarren
            }
        } else {
            Gamemode::MaryWarren
        }
    }
}
