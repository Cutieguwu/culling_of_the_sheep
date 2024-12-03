#[derive(Debug)]
pub struct Character {
    pub name: &'static str,
    pub standing: Standing
}

#[derive(Debug)]
pub struct Standing {
    pub society: i8,
    pub court: i8,
    pub friends: Vec<&'static People>,
    pub enemies: Vec<&'static People>
}

impl Standing {
    pub fn calculate (mut self: Standing) -> i8 {
        // Drain society points if mostly disliked.
        if self.friends.len() < self.enemies.len() {
            self.society = self.society.clone() - 2
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

#[derive(Debug)]
pub enum People {
    AbigailWilliams(Option<Character>),
    AnnPutnam(Option<Character>),
    BettyParris(Option<Character>),
    DeputyGovernorDanforth(Option<Character>),
    ElizabethProctor(Option<Character>),
    EzekielCheever(Option<Character>),
    FrancisNurse(Option<Character>),
    RebeccaNurse(Option<Character>),
    GilesCorey(Option<Character>),
    GoodyOsborne(Option<Character>),
    JohnProctor(Option<Character>),
    JudgeHathorne(Option<Character>),
    MaryWarren(Option<Character>),
    MarshalHerrick(Option<Character>),
    MarthaCorey(Option<Character>),
    MercyLewis(Option<Character>),
    ReverendParris(Option<Character>),
    ReverendJohnHale(Option<Character>),
    SarahGood(Option<Character>),
    SusannaWalcott(Option<Character>),
    ThomasPutnam(Option<Character>),
    Tituba(Option<Character>)
}

#[derive(Debug)]
pub enum SurvivalStatus {
    PlayerLived,
    PlayerDied
}