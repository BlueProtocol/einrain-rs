use poise::serenity::{
    builder::CreateEmbed,
    utils::Colour,
};

use einrain_utils::{Context, Error};
use super::classes::*;

#[poise::command(slash_command)]
#[doc = "Discover detailed information on classes"]
pub async fn class(
    ctx: Context<'_>,
    #[description = "Class type to find information about"] class_type: poise::Wrapper<ClassChoice>,
) -> Result<(), Error> {
    let class_type = class_type.0;

    let class_data = match class_type {
        ClassChoice::AegisFighter => &*CLASS_AEGIS,
        ClassChoice::BlastArcher => &*CLASS_BLAST,
        ClassChoice::SpellCaster => &*CLASS_SPELL,
        ClassChoice::TwinStriker => &*CLASS_TWIN,
    };

    let color = u32::from_str_radix(class_data.color.as_str(), 16).unwrap();

    let basic_stats = &class_data.stats.level_1.basic_stats;
    let attack_stats = &class_data.stats.level_1.attack;
    let defense_stats = &class_data.stats.level_1.defense;

    let basic_stats_fmt = format!("**(Level 1)**\nStrength: {}\nEndurance: {}\nDexterity: {}\nIntelligence: {}\nSpirit: {}", basic_stats.strength, basic_stats.endurance, basic_stats.dexterity, basic_stats.intelligence, basic_stats.spirit);
    let attack_stats_fmt = format!("**(Level 1)**\nAttack: {}\nCrit chance: {}\nCrit damage: {}", attack_stats.attack, attack_stats.crit_chance, attack_stats.crit_damage);
    let defense_stats_fmt = format!("**(Level 1)**\nDefense power: {}\nRecovery power: {}", defense_stats.defense_power, defense_stats.recovery_power);

    let mut primary_skill = &String::new();
    let mut secondary_skill = &String::new();
    let tactical_skills = &mut String::new();
    let mut utility_skill = &String::new();
    let mut ultimate_skill = &String::new();

    for ClassSkill { name, cooldown: _, r#type, upgrade: _, description: _, icon: _ } in &class_data.skill_tree {
        match r#type.as_str() {
            "primary" => primary_skill = name,
            "secondary" => secondary_skill = name,
            "tactical" => {
                tactical_skills.push_str(name.as_str());
                tactical_skills.push('\n');
            },
            "utility" => utility_skill = name,
            "ultimate" => ultimate_skill = name,
            _ => panic!("Invalid skill type found"),
        }
    }

    let embed = CreateEmbed::default()
        .color(Colour::new(color))
        .author(|ae| {
            ae.url("https://blue-protocol-db.com/");
            ae.icon_url("https://i.imgur.com/uwvf8BL.png");
            ae.name("Blue Protocol DB")
        })
        .thumbnail(&class_data.information.image)
        .url(&class_data.url)
        .title(&class_data.name)
        .description(&class_data.name_jp)
        .field("DESCRIPTION", &class_data.information.overview, false)
        .fields(vec![
            ("BASIC STATS", basic_stats_fmt, true),
            ("ATTACK STATS", attack_stats_fmt, true),
            ("DEFENSE STATS", defense_stats_fmt, true),
        ])
        .fields(vec![
            ("Primary Skill", primary_skill, true),
            ("Secondary Skill", secondary_skill, true),
            ("Tactical Skills", tactical_skills, true),
        ])
        .fields(vec![
            ("Utility Skill", utility_skill, true),
            ("Ultimate Skill", ultimate_skill, true),
            ("⁣", &String::from("⁣\n\n\n⁣"), true), // Invisible separator used
        ])
        .field("DISCLAIMER", "This data is subject to change during the CBT of the game.", false)
        .footer(|f| {
            f.text("Made with ♥ by • Blue Protocol DB Team")
        }).to_owned();

    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            *e = embed;
            e
        })
    }).await?;
    Ok(())
}