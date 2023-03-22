//use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
//use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum HairColor {
    Black,
    Brown,
    Red,
    Blonde,
    Blue,
    Pink,
    Purple,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum EyeColor {
    Blue,
    Green,
    Hazel,
    Brown,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SkinColor {
    Light,
    Medium,
    Dark,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Class {
    Mercenary,
    Hacker,
    Rogue, // TODO: rename to something more cyberpunk
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stats {
    // primary stats
    pub level: u32,
    pub experience: u32,
    pub strength: u32,
    pub intelligence: u32,
    pub dexterity: u32,

    // strength stats
    pub hp: u32,
    pub defense: u32,
    pub damage: u32,

    // intelligence stats
    pub ap: u32,
    pub tech_defense: u32,
    pub tech_damage: u32,
    pub tech_critical: u32,

    // dexterity stats
    pub hit_chance: u32,
    pub critical_chance: u32,
    pub dodge_chance: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub sex: Sex,
    pub hair_color: HairColor,
    pub eye_color: EyeColor,
    pub skin_color: SkinColor,
    pub class: Class,
    pub stats: Stats,
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn it_works() {
    //    let result = add(2, 2);
    //    assert_eq!(result, 4);
    //}
}
