mod characters;
mod events;
mod utils;

use characters::{
    Character,
    People,
    SurvivalStatus,
};

use events::{
    EventHandle,
    EventReturn,
    Events,
    Trial,
};

use rand::{
    random,
    thread_rng,
    rngs::ThreadRng
};

use utils::{
    fmt_args,
    ArgType,
    FlagType,
};

//Number of game rounds.
const ROUNDS: u8 = 15;

fn main() {
    let cmd_args: Vec<ArgType> = fmt_args();
    let mut characters: Vec<People> = People::build_characters();
    let rng: ThreadRng = thread_rng();

    // Set player default values based on gamemode.
    let mut player: Character = Character::build_player(Gamemode::get(&cmd_args));
    let mut end_status: SurvivalStatus = SurvivalStatus::PlayerLived;
    
    // Game loop.
    for r in 0..=ROUNDS {
        end_status = run_event(
            &rng,
            &mut player,
            &mut characters,
            random(),
        );

        if matches!(end_status, SurvivalStatus::PlayerLived) { break }

        if characters.len() == 0 {
            end_status = SurvivalStatus::PlayerMassacred
        } else if r == ROUNDS {
            end_status = SurvivalStatus::PlayerLived
        } else if player.standing.clone().calculate() <= 0 {
            //Standing too low, bring to court.
            end_status = run_event(
                &rng,
                &mut player,
                &mut characters,
                Events::Trial(Trial),
            )
        };
        
    };

    match end_status {
        SurvivalStatus::PlayerDied => println!("You died. And for what?"),
        SurvivalStatus::PlayerLived => println!("You survived, but at what cost? How many died to save you?"),
        SurvivalStatus::PlayerMassacred => println!("You killed everyone. I ask, at what end will you yield. Care you not for the life of others?"),
    };
}

fn run_event<'main>(
    rng: &ThreadRng ,
    player: &mut Character,
    characters: &mut Vec<People>,
    event: Events,
) -> SurvivalStatus {
    match event {
        Events::Trial(trial) => match trial.handle(rng, player, characters) {
            EventReturn::Survival(state) => state,
            _ => unreachable!(),
        },
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
    GoodyOsborne,
}

impl Gamemode {
    fn get(cmd_args: &Vec<ArgType>) -> Gamemode {
        if cmd_args.len() != 1 {
            match &cmd_args[1] {
                ArgType::Command(command) => match command.as_str() {
                    "abigail" => Gamemode::Abigail,
                    "john_proctor" => Gamemode::JohnProctor,
                    "goody_osborne" => Gamemode::GoodyOsborne,
                    _ => Gamemode::MaryWarren,
                },
                ArgType::Flag(flag) => match flag {
                    FlagType::Long(flag_long) => match flag_long.as_str() {
                        "abigail" => Gamemode::Abigail,
                        "john-proctor" => Gamemode::JohnProctor,
                        "goody-osborne" => Gamemode::GoodyOsborne,
                        _ => Gamemode::MaryWarren,
                    },
                    FlagType::Short(flag_short) => match flag_short.as_str() {
                        "a" => Gamemode::Abigail,
                        "j" => Gamemode::JohnProctor,
                        "o" => Gamemode::GoodyOsborne,
                        _ => Gamemode::MaryWarren,
                    },
                },
                _ => Gamemode::MaryWarren,
            }
        } else {
            Gamemode::MaryWarren
        }
    }
}
