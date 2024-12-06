use std::i8;

use rand::{
    distributions::{Distribution, Standard},
    rngs::ThreadRng,
};

use crate::characters::{
    Character,
    People,
    SurvivalStatus,
};

#[allow(unused_variables)]
pub trait EventHandle {
    fn handle<'a>(
        self: &Self,
        rng: &'a ThreadRng,
        player: &'a mut Character,
        characters: &'a mut Vec<People>,
    ) -> EventReturn {
        EventReturn::None
    }
}

#[derive(Debug)]
pub struct Trial;

impl EventHandle for Trial {
    fn handle<'a>(
        self: &Self,
        rng: &'a ThreadRng,
        player: &'a mut Character,
        characters: &'a mut Vec<People>,
    ) -> EventReturn {
        let mut player_status: SurvivalStatus = SurvivalStatus::PlayerDied;

        println!("You have been summoned before the court.\n");

        if player.standing.clone().calculate() >= {0 as i8} {
            player_status = SurvivalStatus::PlayerLived
        }

        EventReturn::Survival(player_status)
    }
}

#[derive(Debug)]
pub struct WildAccusation;

impl EventHandle for WildAccusation {
    fn handle<'a>(
        self: &Self,
        rng: &'a ThreadRng,
        player: &'a mut Character,
        characters: &'a mut Vec<People>,
    ) -> EventReturn {
        EventReturn::None
    }
}

#[derive(Debug)]
pub enum Events {
    Trial(Trial),
}

impl Distribution<Events> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Events {
        match rng.gen_range(0..=0) {
            0 => Events::Trial(Trial),
            1 => todo!(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum EventReturn {
    Survival(SurvivalStatus),
    None,
 }