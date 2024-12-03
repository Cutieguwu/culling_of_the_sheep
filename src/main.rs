mod characters;
mod events;
mod utils;

use characters::{
    Character,
    People,
    Standing,
    SurvivalStatus
};
use events::get_event;
use std::vec;
use utils::{
    fmt_args,
    ArgType,
    FlagType
};

//Number of game rounds.
const ROUNDS: u8 = 15;

fn main() {
    let cmd_args = dbg!(fmt_args());
    let mut characters = build_characters();

    // Set player default values based on gamemode.
    let mut player = dbg!(build_player(dbg!(get_gamemode(cmd_args))));
    let end_status = SurvivalStatus::PlayerLived;

    for _r in 0..=ROUNDS {
        get_event().handle(player);

        if player.standing.calculate() <= 0 {
            let mut trial = events::Trial;

            match trial.handle(&mut player) {
                SurvivalStatus::PlayerDied => {
                    end_status = SurvivalStatus::PlayerDied
                },
                _ => ()
            }
        }
    };
}

#[derive(Debug)]
enum Gamemode {
    //Easy
    Abigail,
    //Normal
    MaryWarren,
    //Hard
    JohnProctor,
    //Just dead.
    GoodyOsborne
}

fn build_characters() -> Vec<People> {
    vec![
        People::AbigailWilliams(Some(Character {
            name: "Abigail Williams",
            standing: Standing {
                society: 10,
                court: 10,
                friends: vec![
                    &People::BettyParris(None),
                    &People::DeputyGovernorDanforth(None),
                    &People::ReverendParris(None),
                ],
                enemies: vec![
                    &People::ElizabethProctor(None),
                    &People::JohnProctor(None),
                    &People::ReverendJohnHale(None)
                ]
            }
        })),
        People::BettyParris(Some(Character {
            name: "Betty Parris",
            standing: Standing {
                society: 10,
                court: 10,
                friends: vec![
                    &People::AbigailWilliams(None),
                    &People::ReverendParris(None)
                ],
                enemies: vec![
                    &People::JohnProctor(None),
                    &People::ElizabethProctor(None)
                ]
            }
        })),
        People::ElizabethProctor(Some(Character{
            name: "Elizabeth Proctor",
            standing: Standing {
                society: 10,
                court: 5,
                friends: vec![
                    &People::FrancisNurse(None),
                    &People::GilesCorey(None),
                    &People::JohnProctor(None),
                    &People::RebeccaNurse(None),
                    &People::ReverendJohnHale(None)
                ],
                enemies: vec![
                    &People::AbigailWilliams(None),
                    &People::DeputyGovernorDanforth(None),
                    &People::EzekielCheever(None)
                ]
            }
        }))
    ]
}

fn build_player(gamemode: Gamemode) -> Character {
    Character {
        name: "You",
        standing: match gamemode {
            Gamemode::Abigail => Standing {
                society: 10,
                court: 10,
                friends: vec![],
                enemies: vec![]
            },
            Gamemode::MaryWarren => Standing {
                society: 5,
                court: 5,
                friends: vec![],
                enemies: vec![]
            },
            Gamemode::JohnProctor => Standing {
                society: 2,
                court: 2,
                friends: vec![],
                enemies: vec![]
            },
            Gamemode::GoodyOsborne => Standing {
                society: -15,
                court: -5,
                friends: vec![],
                enemies: vec![]
            }
        }
    }
}

fn get_gamemode(cmd_args: Vec<ArgType>) -> Gamemode {
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