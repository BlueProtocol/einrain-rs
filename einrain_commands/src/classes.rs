use std::{cmp::Ordering, include_str};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Debug, poise::SlashChoiceParameter)]
pub enum ClassChoice {
    #[name = "Aegis Fighter"]
    AegisFighter,
    #[name = "Blast Archer"]
    BlastArcher,
    #[name = "Spell Caster"]
    SpellCaster,
    #[name = "Twin Striker"]
    TwinStriker,
}

#[derive(Serialize, Deserialize)]
pub struct ClassType {
    pub name: String,
    pub name_jp: String,
    pub color: String,
    pub url: String,
    pub information: ClassInformation,
    pub stats: ClassStats,
    pub skill_tree: Vec<ClassSkill>,
}

#[derive(Serialize, Deserialize)]
pub struct ClassInformation {
    pub overview: String,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClassStats {
    pub level_1: ClassLevelStats,
}

#[derive(Serialize, Deserialize)]
pub struct ClassLevelStats {
    pub basic_stats: ClassBasicStats,
    pub attack: ClassAttackStats,
    pub defense: ClassDefenseStats,
}

#[derive(Serialize, Deserialize)]
pub struct ClassBasicStats {
    pub strength: String,
    pub endurance: String,
    pub dexterity: String,
    pub intelligence: String,
    pub spirit: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClassAttackStats {
    pub attack: String,
    pub crit_chance: String,
    pub crit_damage: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClassDefenseStats {
    pub defense_power: String,
    pub recovery_power: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ClassSkill {
    pub name: String,
    pub cooldown: Option<String>,
    pub r#type: String,
    pub upgrade: String,
    pub description: String,
    pub icon: String,
}

impl PartialOrd for ClassSkill {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s = match self.r#type.as_str() {
            "primary" => 6,
            "secondary" => 5,
            "utility" => 4,
            "tactical" => 3,
            "ultimate" => 2,
            _ => 1
        };

        let o = match other.r#type.as_str() {
            "primary" => 6,
            "secondary" => 5,
            "utility" => 4,
            "tactical" => 3,
            "ultimate" => 2,
            _ => 1
        };

        s.partial_cmp(&o)
    }
}

impl Ord for ClassSkill {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = match self.r#type.as_str() {
            "primary" => 6,
            "secondary" => 5,
            "utility" => 4,
            "tactical" => 3,
            "ultimate" => 2,
            _ => 1
        };

        let o = match other.r#type.as_str() {
            "primary" => 6,
            "secondary" => 5,
            "utility" => 4,
            "tactical" => 3,
            "ultimate" => 2,
            _ => 1
        };

        s.cmp(&o)
    }
}

static CLASS_AEGIS_JSON: &str = include_str!("assets\\classes\\aegis_fighter.json");
static CLASS_BLAST_JSON: &str = include_str!("assets\\classes\\blast_archer.json");
static CLASS_SPELL_JSON: &str = include_str!("assets\\classes\\spell_caster.json");
static CLASS_TWIN_JSON: &str = include_str!("assets\\classes\\twin_striker.json");

lazy_static! {
    pub static ref CLASS_AEGIS: ClassType = serde_json::from_str(&CLASS_AEGIS_JSON).unwrap();
    pub static ref CLASS_BLAST: ClassType = serde_json::from_str(&CLASS_BLAST_JSON).unwrap();
    pub static ref CLASS_SPELL: ClassType = serde_json::from_str(&CLASS_SPELL_JSON).unwrap();
    pub static ref CLASS_TWIN: ClassType = serde_json::from_str(&CLASS_TWIN_JSON).unwrap();
}