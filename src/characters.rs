use rand::distributions::{Distribution, Standard};

use crate::Gamemode;

#[derive(Debug, Clone)]
pub struct Character {
    pub name: &'static str,
    pub standing: Standing,
}

impl Character {
    pub fn build_player(gamemode: Gamemode) -> Character {
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
}

#[derive(Debug, Clone)]
pub struct Standing {
    pub society: i8,
    pub court: i8,
    pub friends: Vec<&'static People>,
    pub enemies: Vec<&'static People>,
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
    GilesCorey(Option<Character>),
    GoodyOsborne(Option<Character>),
    JohnProctor(Option<Character>),
    JudgeHathorne(Option<Character>),
    MaryWarren(Option<Character>),
    MarshalHerrick(Option<Character>),
    MarthaCorey(Option<Character>),
    MercyLewis(Option<Character>),
    RebeccaNurse(Option<Character>),
    ReverendParris(Option<Character>),
    ReverendJohnHale(Option<Character>),
    SarahGood(Option<Character>),
    SusannaWalcott(Option<Character>),
    ThomasPutnam(Option<Character>),
    Tituba(Option<Character>),
}

impl People {
    pub fn build_characters() -> Vec<People> {
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
                        &People::ReverendJohnHale(None),
                    ]
                }
            })),
            People::AnnPutnam(Some(Character {
                name: "Ann Putnam",
                standing: Standing {
                    society: 10,
                    court: 7,
                    friends: vec![
                        &People::BettyParris(None),
                        &People::ThomasPutnam(None),
                        &People::ReverendParris(None),
                        &People::MercyLewis(None),
                    ],
                    enemies: vec![
                        &People::ElizabethProctor(None),
                        &People::JohnProctor(None),
                        &People::RebeccaNurse(None),
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
                        &People::ReverendParris(None),
                    ],
                    enemies: vec![
                        &People::JohnProctor(None),
                        &People::ElizabethProctor(None),
                    ]
                }
            })),
            People::DeputyGovernorDanforth(Some(Character {
                name: "Deputy Governor Danforth",
                standing: Standing {
                    society: 10,
                    court: 100,
                    friends: vec![
                        &People::AbigailWilliams(None),
                        &People::JudgeHathorne(None),
                        &People::ReverendParris(None),
                        &People::EzekielCheever(None),
                        &People::MarshalHerrick(None),
                    ],
                    enemies: vec![
                        &People::ElizabethProctor(None),
                        &People::JohnProctor(None),
                        &People::ReverendJohnHale(None),
                        &People::FrancisNurse(None),
                        &People::GilesCorey(None),
                        &People::MarthaCorey(None),
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
                        &People::ReverendJohnHale(None),
                    ],
                    enemies: vec![
                        &People::AbigailWilliams(None),
                        &People::DeputyGovernorDanforth(None),
                        &People::EzekielCheever(None),
                    ]
                }
            })),
            People::EzekielCheever(Some(Character {
                name: "Ezekiel Cheever",
                standing: Standing {
                    society: 12,
                    court: 50,
                    friends: vec![
                        &People::MarshalHerrick(None),
                        &People::DeputyGovernorDanforth(None),
                        &People::ReverendParris(None),
                        &People::ReverendJohnHale(None),
                        &People::JudgeHathorne(None),
                
                    ],
                    enemies: vec![
                        &People::ElizabethProctor(None),
                        &People::JohnProctor(None),
                        &People::FrancisNurse(None),
                        &People::GilesCorey(None),
                    ]
                }
            })),
            People::FrancisNurse(Some(Character {
                name: "Francis Nurse",
                standing: Standing {
                    society: 10,
                    court: 10,
                    friends: vec![
                        &People::RebeccaNurse(None),
                        &People::JohnProctor(None),
                        &People::GilesCorey(None),
                    ],
                    enemies: vec![
                        &People::AnnPutnam(None),
                        &People::AbigailWilliams(None),
                        &People::ReverendJohnHale(None),
                        &People::JudgeHathorne(None),
                        &People::ThomasPutnam(None),
                    ]
                }
            })),
            People::RebeccaNurse(Some(Character {
                name: "Rebecca Nurse",
                standing: Standing {
                    society: 20,
                    court: 8,
                    friends: vec![
                        &People::ElizabethProctor(None),
                        &People::FrancisNurse(None),
                        &People::JohnProctor(None),
                        &People::MarthaCorey(None),
                        &People::GilesCorey(None),
                    ],
                    enemies: vec![
                        &People::ThomasPutnam(None),
                        &People::AnnPutnam(None),
                        &People::ReverendJohnHale(None),
                        &People::JudgeHathorne(None),
                        &People::EzekielCheever(None),
                        &People::MarshalHerrick(None),
                        &People::AbigailWilliams(None),
                    ]
                }
            })),
            People::GilesCorey(Some(Character {
                name: "Giles Corey",
                standing: Standing {
                    society: 2,
                    court: 5,
                    friends: vec![
                        &People::JohnProctor(None),
                        &People::MarthaCorey(None),
                        &People::FrancisNurse(None),
                    ],
                    enemies: vec![
                        &People::EzekielCheever(None),
                        &People::JudgeHathorne(None),
                        &People::ReverendJohnHale(None),
                        &People::MarshalHerrick(None),
                    ]
                }
            })),
        ]
    }
}

impl Distribution<People> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> People {
        match rng.gen_range(0..=21) {
            0 => People::AbigailWilliams(None),
            1 => People::AnnPutnam(None),
            2 => People::BettyParris(None),
            3 => People::DeputyGovernorDanforth(None),
            4 => People::ElizabethProctor(None),
            5 => People::EzekielCheever(None),
            6 => People::FrancisNurse(None),
            7 => People::GilesCorey(None),
            8 => People::GoodyOsborne(None),
            9 => People::JohnProctor(None),
            10 => People::JudgeHathorne(None),
            11 => People::MarshalHerrick(None),
            12 => People::MarthaCorey(None),
            13 => People::MaryWarren(None),
            14 => People::RebeccaNurse(None),
            15 => People::ReverendJohnHale(None),
            16 => People::ReverendParris(None),
            18 => People::SarahGood(None),
            19 => People::SusannaWalcott(None),
            20 => People::ThomasPutnam(None),
            21 => People::Tituba(None),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum SurvivalStatus {
    PlayerMassacred,
    PlayerLived,
    PlayerDied,
}