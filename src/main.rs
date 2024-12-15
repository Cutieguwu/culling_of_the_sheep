// Copyright (c) 2024 Cutieguwu | Olivia Brooks
//
// -*- coding: utf-8 -*-
// @Title: Culling of the Sheep
// @Author: Cutieguwu | Olivia Brooks
// @Description: A game to demonstrate the Salem Witch Trials.
//
// @Script: main.rs
// @Date Created: 26 Nov, 2024
// @Last Modified: 09 Dec, 2024
// @Last Modified by: Cutieguwu | Olivia Brooks
// --------------------------------------------


mod characters;
mod events;
mod utils;

use rand::{random, thread_rng};
use rand::rngs::ThreadRng;

use utils::{fmt_args, ArgType, FlagType};

use crate::characters::{Character, People, SurvivalStatus};
use crate::events::{EventHandle, Events, print_breakline};

//Number of game rounds.
const ROUNDS: u8 = 15;

fn main() {
    let mut characters: Vec<Character> = People::load_characters();
    let mut end_status: SurvivalStatus = SurvivalStatus::PlayerLived;
    // Set player default values based on gamemode.
    let mut player: Character = Character::build_player(
        &mut characters,
        Gamemode::get(fmt_args()),
    );
    let mut rng: ThreadRng = thread_rng();

    // Game loop.
    for r in 0..=ROUNDS {
        end_status = random::<Events>().handle(
            &mut rng,
            &mut player,
            &mut characters,
        );

        if characters.len() == 1 {
            end_status = SurvivalStatus::PlayerMassacred;
        };

        if end_status == SurvivalStatus::PlayerDied
            || end_status == SurvivalStatus::PlayerMassacred
            || r == ROUNDS
        {
            break
        }
    };

    print_breakline();

    // Game end result.
    match end_status {
        SurvivalStatus::PlayerDied => println!("You died. And for what?"),
        SurvivalStatus::PlayerLived => {
            println!("You survived, but at what cost?");
            println!("How many died to save you?");
        },
        SurvivalStatus::PlayerMassacred => {
            println!("You killed everyone.");
            println!("I ask, at what end will you yield.");
            println!("Care you not for the life of others?");
        },
    };
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
    fn get(cmd_args: Vec<ArgType>) -> Gamemode {
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
