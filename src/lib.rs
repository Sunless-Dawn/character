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

impl Stats {
    pub fn new() -> Self {
        let mut s = Self {
            level: 0,
            experience: 0,
            strength: 5000,
            intelligence: 5000,
            dexterity: 5000,
            hp: 0,
            defense: 0,
            damage: 0,
            ap: 0,
            tech_defense: 0,
            tech_damage: 0,
            tech_critical: 0,
            hit_chance: 0,
            critical_chance: 0,
            dodge_chance: 0,
        };

        s.update();

        s
    }

    pub fn level_up(&mut self, class: &Class) {
        self.level += 1;
        self.experience = 0;

        // increase primary stat
        match class {
            Class::Mercenary => {
                self.strength += 2000;
                self.intelligence += 1000;
                self.dexterity += 1000;
            }
            Class::Hacker => {
                self.strength += 1000;
                self.intelligence += 2000;
                self.dexterity += 1000;
            }
            Class::Rogue => {
                self.strength += 1000;
                self.intelligence += 1000;
                self.dexterity += 2000;
            }
        }

        // recalculate secondary stats
        self.update();
    }

    pub fn update(&mut self) {
        let starting_hp = 100000; // 100 HP to start
        let hp_growth = 5000; // grows by 5 per level
        let starting_damage = 10000; // 10 damage to start, grows by strength
        let defense_ratio = 4; // 25% of strength is defense

        let starting_ap = 20000; // 20 AP to start
        let ap_growth = 2000; // grows by 2 per level
        let tech_defense_ratio = 4; // 25% of intelligence is tech_defense
        let tech_critical_ratio = 4; // 25% of intelligence is tech_critical
        let tech_starting_damage = 10000; // 10 tech_damage to start, grows by intelligence

        let starting_hit_chance = 50000; // 50% hit chance
        let hit_chance_growth = 7; // numerator of hit chance % growth
        let hit_chance_ratio = 100; // denominator of hit chance % growth
        let critical_chance_ratio = 4; // 25% of dexterity is critical_chance
        let dodge_chance_ratio = 4; // 25% of dexterity is dodge_chance

        // strength stats
        self.hp = starting_hp + (hp_growth * self.level);
        self.defense = self.strength / defense_ratio;
        self.damage = starting_damage + self.strength;

        // intelligence stats
        self.ap = starting_ap + (ap_growth * self.level);
        self.tech_defense = self.intelligence / tech_defense_ratio;
        self.tech_damage = tech_starting_damage + self.intelligence;
        self.tech_critical = self.intelligence / tech_critical_ratio;

        // dexterity stats
        self.hit_chance =
            starting_hit_chance + (hit_chance_growth * self.dexterity / hit_chance_ratio);
        self.critical_chance = self.dexterity / critical_chance_ratio;
        self.dodge_chance = self.dexterity / dodge_chance_ratio;
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub sex: Sex,
    pub hair_color: HairColor,
    pub eye_color: EyeColor,
    pub skin_color: SkinColor,
    pub stats: Stats,
}

impl Character {
    pub fn new(
        name: String,
        class: Class,
        sex: Sex,
        hair_color: HairColor,
        eye_color: EyeColor,
        skin_color: SkinColor,
    ) -> Self {
        Self {
            name,
            class,
            sex,
            hair_color,
            eye_color,
            skin_color,
            stats: Stats::new(),
        }
    }

    pub fn level_up(&mut self) {
        self.stats.level_up(&self.class);
    }
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
