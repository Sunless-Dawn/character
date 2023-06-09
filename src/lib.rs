//! This crate providers the character library for Sunless Dawn

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

/// Character Sex. Male or Female.
#[derive(Clone, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
}

impl Distribution<Sex> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sex {
        match rng.gen_range(0..=1) {
            0 => Sex::Male,
            _ => Sex::Female,
        }
    }
}

/// Character hair color choices
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

impl Distribution<HairColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HairColor {
        match rng.gen_range(0..=6) {
            0 => HairColor::Black,
            1 => HairColor::Brown,
            2 => HairColor::Red,
            3 => HairColor::Blonde,
            4 => HairColor::Blue,
            5 => HairColor::Pink,
            _ => HairColor::Purple,
        }
    }
}

/// Character eye color choices
#[derive(Clone, Serialize, Deserialize)]
pub enum EyeColor {
    Blue,
    Green,
    Hazel,
    Brown,
}

impl Distribution<EyeColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EyeColor {
        match rng.gen_range(0..=3) {
            0 => EyeColor::Blue,
            1 => EyeColor::Green,
            2 => EyeColor::Hazel,
            _ => EyeColor::Brown,
        }
    }
}

/// Character skin color choices
#[derive(Clone, Serialize, Deserialize)]
pub enum SkinColor {
    Light,
    Medium,
    Dark,
}

impl Distribution<SkinColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkinColor {
        match rng.gen_range(0..=2) {
            0 => SkinColor::Light,
            1 => SkinColor::Medium,
            _ => SkinColor::Dark,
        }
    }
}

/// Character class choices
/// Mercenary has a primary stat of Strength
/// Hacker has a primary stat of Intelligence
/// Rogue has a primary stat of Dexterity
#[derive(Clone, Serialize, Deserialize)]
pub enum Class {
    Mercenary,
    Hacker,
    Rogue, // TODO: rename to something more cyberpunk
}

impl Distribution<Class> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Class {
        match rng.gen_range(0..=2) {
            0 => Class::Mercenary,
            1 => Class::Hacker,
            _ => Class::Rogue,
        }
    }
}

/// Character stats
/// level, strength, intelligence, and dexterity are the primary stats
/// the other stats are derived as a combination of level and primary stats
/// items equipped to the character can affect stats as well
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

/// Default function for Stats, default values for a new Character
impl Default for Stats {
    fn default() -> Self {
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
}

impl Stats {
    /// Stats::new returns a default Stats
    pub fn new() -> Self {
        Default::default()
    }

    /// level_up increases the level of this Stats, and recalculates stats accordingly
    /// level_up also resets experience back to 0
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

    /// update() recalculates all of the secondary stats from the primary stats
    /// this function should be called whenever there is a change in level or equipment
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
        let hit_chance_growth = 228; // hit chance growth per level
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
        self.hit_chance = starting_hit_chance + (hit_chance_growth * self.level);
        self.critical_chance = self.dexterity / critical_chance_ratio;
        self.dodge_chance = self.dexterity / dodge_chance_ratio;

        // at level 0 use starting values
        if self.level == 0 {
            self.damage = starting_damage;
            self.tech_damage = starting_damage;
            self.hit_chance = starting_hit_chance;
        }
    }
}

/// Character struct represents a character
/// There are several immutable characteristics like name, class, and physical traits
/// as well as the characters Stats
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
    /// returns a new character with the parameters specified
    pub fn new(
        name: &'static str,
        class: Class,
        sex: Sex,
        hair_color: HairColor,
        eye_color: EyeColor,
        skin_color: SkinColor,
    ) -> Self {
        Self {
            name: name.to_string(),
            class,
            sex,
            hair_color,
            eye_color,
            skin_color,
            stats: Stats::new(),
        }
    }

    // return a character with random physical traits
    pub fn random(name: &'static str) -> Self {
        Self {
            name: name.to_string(),
            class: rand::random(),
            sex: rand::random(),
            hair_color: rand::random(),
            eye_color: rand::random(),
            skin_color: rand::random(),
            stats: Stats::new(),
        }
    }

    /// increase this character's level by one
    pub fn level_up(&mut self) {
        self.stats.level_up(&self.class);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test default values for stats
    #[test]
    fn test_default_stats() {
        let s: Stats = Default::default();
        assert_eq!(s.level, 0);
        assert_eq!(s.experience, 0);
        assert_eq!(s.strength, 5000);
        assert_eq!(s.intelligence, 5000);
        assert_eq!(s.dexterity, 5000);
        assert_eq!(s.hp, 100000);
        assert_eq!(s.defense, 1250);
        assert_eq!(s.damage, 10000);
        assert_eq!(s.ap, 20000);
        assert_eq!(s.tech_damage, 10000);
        assert_eq!(s.tech_defense, 1250);
        assert_eq!(s.tech_critical, 1250);
        assert_eq!(s.hit_chance, 50000);
        assert_eq!(s.critical_chance, 1250);
        assert_eq!(s.dodge_chance, 1250);
    }

    #[test]
    fn test_level_up() {
        let mut c = Character::new(
            "Test Character",
            Class::Mercenary,
            Sex::Male,
            HairColor::Brown,
            EyeColor::Hazel,
            SkinColor::Light,
        );
        c.level_up();

        {
            let s = &c.stats;
            assert_eq!(s.level, 1);
            assert_eq!(s.experience, 0);
            assert_eq!(s.strength, 7000);
            assert_eq!(s.intelligence, 6000);
            assert_eq!(s.dexterity, 6000);
            assert_eq!(s.hp, 105000);
            assert_eq!(s.defense, 1750);
            assert_eq!(s.damage, 17000);
            assert_eq!(s.ap, 22000);
            assert_eq!(s.tech_damage, 16000);
            assert_eq!(s.tech_defense, 1500);
            assert_eq!(s.tech_critical, 1500);
            assert_eq!(s.hit_chance, 50228);
            assert_eq!(s.critical_chance, 1500);
            assert_eq!(s.dodge_chance, 1500);
        }

        // level up to 100
        for _ in 0..99 {
            c.level_up();
        }

        {
            let s = &c.stats;
            assert_eq!(s.level, 100);
            assert_eq!(s.experience, 0);
            assert_eq!(s.strength, 205000);
            assert_eq!(s.intelligence, 105000);
            assert_eq!(s.dexterity, 105000);
            assert_eq!(s.hp, 600000);
            assert_eq!(s.defense, 51250);
            assert_eq!(s.damage, 215000);
            assert_eq!(s.ap, 220000);
            assert_eq!(s.tech_damage, 115000);
            assert_eq!(s.tech_defense, 26250);
            assert_eq!(s.tech_critical, 26250);
            assert_eq!(s.hit_chance, 72800);
            assert_eq!(s.critical_chance, 26250);
            assert_eq!(s.dodge_chance, 26250);
        }
    }
}
