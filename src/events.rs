use crate::characters::{
    Character,
    SurvivalStatus
};

pub fn get_event() {}

pub struct Trial;

impl Trial {
    pub fn handle(&self, &mut player: &mut Character) -> SurvivalStatus {
        let player_status = SurvivalStatus::PlayerDied;

        player_status
    }
}
