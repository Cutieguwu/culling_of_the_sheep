// Copyright (c) 2024 Cutieguwu | Olivia Brooks
//
// -*- coding: utf-8 -*-
// @Title: Culling of the Sheep
// @Author: Cutieguwu | Olivia Brooks
// @Description: Components for characters.
//
// @Script: characters.rs
// @Date Created: 03 Dec, 2024
// @Last Modified: 10 Dec, 2024
// @Last Modified by: Cutieguwu | Olivia Brooks
// --------------------------------------------

use std::fs::File;

use rand::distributions::{Distribution, Standard};
use ron::de::from_reader;
use serde::Deserialize;

use crate::Gamemode;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Character {
    pub id: People,
    pub name: String,
    pub standing: Standing,
}

impl Character {
    pub fn build_player<'a>(
            characters: &'a mut Vec<Character>,
            gamemode: Gamemode,
        ) -> Character {
        let character_model: People = match gamemode {
            Gamemode::Abigail => People::AbigailWilliams,
            Gamemode::MaryWarren => People::MaryWarren,
            Gamemode::JohnProctor => People::JohnProctor,
            Gamemode::GoodyOsborne => People::GoodyOsborne,
        };
        let position: usize = characters.iter()
            .position(|x| x.id == character_model)
            .expect("Failed to locate character to model player by");

        characters.swap_remove(position)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Standing {
    pub society: i8,
    pub court: i8,
    pub friends: Vec<People>,
    pub enemies: Vec<People>,
}

impl Standing {
    pub fn calculate (mut self: Self) -> i8 {
        // Drain society points if mostly disliked.
        if self.friends.len() < self.enemies.len() {
            self.society = &self.society - 2
        }

        // Points
        {
            self.friends.len() as i8
            - self.enemies.len() as i8
            + self.court
            + self.society
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum People {
    AbigailWilliams,
    AnnPutnam,
    BettyParris,
    DeputyGovernorDanforth,
    ElizabethProctor,
    EzekielCheever,
    FrancisNurse,
    GilesCorey,
    GoodyOsborne,
    JohnProctor,
    JudgeHathorne,
    MaryWarren,
    MarshalHerrick,
    MarthaCorey,
    MercyLewis,
    RebeccaNurse,
    ReverendJohnHale,
    ReverendParris,
    SarahGood,
    SusannaWalcott,
    ThomasPutnam,
    Tituba,
    Player,
}

impl People {
    pub fn load_characters() -> Vec<Character> {
        let file_path = format!("{}/data/characters.ron", env!("CARGO_MANIFEST_DIR"));
        let file = File::open(file_path).expect("Error opening character file");

        // Return deserialized file content.
        match from_reader(file) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);

                std::process::exit(1);
            }
        }
    }
}

impl Distribution<People> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> People {
        match rng.gen_range(0..=21) {
            0 => People::AbigailWilliams,
            1 => People::AnnPutnam,
            2 => People::BettyParris,
            3 => People::DeputyGovernorDanforth,
            4 => People::ElizabethProctor,
            5 => People::EzekielCheever,
            6 => People::FrancisNurse,
            7 => People::GilesCorey,
            8 => People::GoodyOsborne,
            9 => People::JohnProctor,
            10 => People::JudgeHathorne,
            11 => People::MarshalHerrick,
            12 => People::MarthaCorey,
            13 => People::MaryWarren,
            14 => People::RebeccaNurse,
            15 => People::ReverendJohnHale,
            16 => People::ReverendParris,
            18 => People::SarahGood,
            19 => People::SusannaWalcott,
            20 => People::ThomasPutnam,
            21 => People::Tituba,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SurvivalStatus {
    PlayerMassacred,
    PlayerLived,
    PlayerDied,
}