// Copyright (c) 2024 Cutieguwu | Olivia Brooks
//
// -*- coding: utf-8 -*-
// @Title: Culling of the Sheep
// @Author: Cutieguwu | Olivia Brooks
// @Description: Components for events.
//
// @Script: events.rs
// @Date Created: 03 Dec, 2024
// @Last Modified: 08 Dec, 2024
// @Last Modified by: Cutieguwu | Olivia Brooks
// --------------------------------------------


use std::i8;

use rand::distributions::{Distribution, Standard};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::characters::{Character, SurvivalStatus};
use crate::utils::input;

const BREAKLINE: &'static str = "-------------------------------------";

pub fn print_breakline() {
    println!("\n{}\n", BREAKLINE)
}

#[allow(unused_variables)]
pub trait EventHandle {
    fn handle<'a>(
        self: &Self,
        rng: &'a mut ThreadRng,
        player: &'a mut Character,
        characters: &'a mut Vec<Character>,
    ) -> SurvivalStatus {
        todo!()
    }
}

#[derive(Debug)]
pub struct Trial;

impl EventHandle for Trial {
    fn handle<'a>(
        self: &Self,
        rng: &'a mut ThreadRng,
        player: &'a mut Character,
        characters: &'a mut Vec<Character>,
    ) -> SurvivalStatus {
        let doppelgangers = characters.clone();

        print_breakline();

        println!("You have been summoned before the court.\n");
        println!("[0] You may appease the court by admitting to witchcraft.");
        println!("[1] Or you may die.");

        // Get choice from player, return early if they choose to die.
        loop {
            match input().as_str() {
                // Attempt to accuse
                "0" => break,
                // Die a witch
                "1" => return SurvivalStatus::PlayerDied,
                _ => ()
            };
        }

        print_breakline();

        println!("Someone must have coerced you into following Satan.");
        println!("Attempt to save yourself; who will you accuse?\n");

        // Randomly select four characters.
        let accusable: Vec<&Character> = doppelgangers
            .choose_multiple(rng, 4)
            .collect();

        for character in &accusable {
            println!(
                "[{:?}] {}",
                accusable
                    .iter()
                    .position(|x| x == character)
                    .expect("Could not find character."),
                character.name
            )
        }

        let accusee_index: u8 = loop {
            let temp: Option<i32> = match input().as_str() {
                "0" => Some(0),
                "1" => Some(1),
                "2" => Some(2),
                "3" => Some(3),
                _ => None
            };

            if temp != None {
                break temp.unwrap() as u8
            }
        };

        push_enemy(
            player,
            accusable[accusee_index as usize],
            characters
        );

        print_breakline();

        println!("The court will now decide your fate considering.");

        // Have the player stand trial.
        // A wild accusation may not be enough to save them.
        if player.standing.clone().calculate() > {0 as i8} {
            SurvivalStatus::PlayerLived
        } else {
            SurvivalStatus::PlayerDied
        }
    }
}

#[derive(Debug)]
pub struct WildAccusation;

impl EventHandle for WildAccusation {
    #[allow(unused_variables)]
    fn handle<'a>(
        self: &Self,
        rng: &'a mut ThreadRng,
        player: &'a mut Character,
        characters: &'a mut Vec<Character>,
    ) -> SurvivalStatus {
        let doppelgangers = characters.clone();
        let accuser: &Character = doppelgangers
            .choose(rng)
            .expect("Failed to fetch character as accuser.");

        let accusee: &Character = doppelgangers
            .choose(rng)
            .expect("Failed to fetch character as accusee.");

        print_breakline();

        println!("{:?} accused {:?}", accuser, accusee);

        push_enemy(accuser, accusee, characters);

        SurvivalStatus::PlayerLived
    }
}

fn push_enemy<'a>(
    accuser: &Character,
    accusee: &Character,
    characters: &'a mut Vec<Character>,
) -> Vec<Character> {
    let characters = characters;
    // If accuser not an enemy, add them as such.
    if None == accusee.standing.enemies
        .iter()
        .find(|x| *x == &accuser.id)
    {
        accusee.standing.enemies.to_owned().push(accuser.id.clone());

        let position: usize = characters.iter()
            .position(|x| x.id == accusee.id)
            .unwrap();

        characters[position] = accusee.clone();
    };

    characters.to_vec()
}

#[derive(Debug)]
pub enum Events {
    Trial(Trial),
    WildAccusation(WildAccusation),
}

impl EventHandle for Events {
    fn handle<'a>(
        self: &Self,
        rng: &'a mut ThreadRng,
        player: &'a mut Character,
        characters: &'a mut Vec<Character>,
    ) -> SurvivalStatus {
        match self {
            Events::Trial(trial) => trial.handle(rng, player, characters),
            Events::WildAccusation(wild_accusation) => {
                wild_accusation.handle(rng, player, characters)
            },
        }
    }
}

impl Distribution<Events> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Events {
        match rng.gen_range(0..=1) {
            0 => Events::Trial(Trial),
            1 => Events::WildAccusation(WildAccusation),
            _ => unreachable!(),
        }
    }
}