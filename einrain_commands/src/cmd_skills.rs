use poise::serenity_prelude::{
    builder::CreateEmbed,
    utils::Colour,
};

use einrain_utils::{Context, Error};
use super::classes::*;

#[poise::command(slash_command)]
#[doc = "Discover detailed information on skills"]
pub async fn skills(
    ctx: Context<'_>,
    #[description = "Class type to find information about"] class_type: ClassChoice,
) -> Result<(), Error> {
    let class_data = match class_type {
        ClassChoice::AegisFighter => &*CLASS_AEGIS,
        ClassChoice::BlastArcher => &*CLASS_BLAST,
        ClassChoice::SpellCaster => &*CLASS_SPELL,
        ClassChoice::TwinStriker => &*CLASS_TWIN,
    };

    let color = u32::from_str_radix(class_data.color.as_str(), 16).unwrap();

    let mut embed = CreateEmbed::default()
        .color(Colour::new(color))
        .author(|ae| {
            ae.url("https://blue-protocol-db.com/");
            ae.icon_url("https://i.imgur.com/uwvf8BL.png");
            ae.name("Blue Protocol DB")
        })
        .thumbnail(&class_data.information.image)
        .url(&class_data.url)
        .title(&(class_data.name.clone() + " Skills"))
        .to_owned();

    let mut skills = class_data.skill_tree.clone();
    skills.sort();
    skills.reverse();

    for ClassSkill { name, cooldown: _, r#type, upgrade: _, description, icon: _ } in skills {
        let mut type_titlecase = r#type.clone();
        if let Some(c) = type_titlecase.get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        embed.field(type_titlecase, format!("**{}**\n{}", name, description), false);
    }
    embed.field("⁣", "⁣\n\n\n\n\n", false);

    embed
        .field("Disclaimer", "This data is subject to change during the CBT of the game.", false)
        .footer(|f| {
            f.text("Made with ♥ by • Blue Protocol DB Team")
        });

    poise::send_reply(ctx, |m| {
        m.embed(|e| {
            *e = embed;
            e
        })
    }).await?;
    Ok(())
}