use spacetimedb::{ReducerContext, Table, log};
use CrewRarity::*;
use CrewTrait::*;
use crate::types::*;
use crate::tables::{CrewTemplate, crew_template};

/// Helper macro to create crew templates with less boilerplate
/// Default values: ability_power=10, attack_speed=1.0, magic_resistance=5
macro_rules! crew {
    ($name:expr, $rarity:expr, [$($trait:expr),*], $hp:expr, $atk:expr, $def:expr, $cost:expr) => {
        CrewTemplate {
            id: 0,
            name: $name.to_string(),
            rarity: $rarity,
            traits: vec![$($trait),*],
            max_hp: $hp,
            ability_power: 10,      // Default AP
            attack: $atk,
            attack_speed: 1.0,      // Default attack speed
            defense: $def,
            magic_resistance: 5,    // Default MR
            cost: $cost,
        }
    };
}

/// Initialize the crew template database - called once on server initialization
pub fn init_crew_templates(ctx: &ReducerContext) {
    // Only initialize if the table is empty
    if ctx.db.crew_template().count() > 0 {
        log::info!("Crew templates already initialized, skipping...");
        return;
    }

    log::info!("Initializing crew template database...");

    let crew_templates = vec![
        // Straw Hats
        crew!("Brook", Uncommon, [StrawHat, Swordsman], 25, 2, 5, 100000),
        crew!("Franky", Rare, [StrawHat, Brawler], 45, 3, 8, 300000),
        crew!("Jimbei", Rare, [StrawHat, Brawler], 60, 3, 8, 400000),
        crew!("Luffy", Legendary, [StrawHat, Emperor, Zoan], 40, 2, 12, 500000),
        crew!("Nami", Uncommon, [StrawHat], 20, 1, 5, 100000),
        crew!("Sanji", Rare, [StrawHat], 35, 3, 8, 300000),
        crew!("Zoro", Rare, [StrawHat, Swordsman], 50, 4, 8, 400000),

        // Red Hair Pirates
        crew!("Beckman", Rare, [RedHairPirates, Sniper], 40, 5, 8, 400000),
        crew!("Lucky Roux", Rare, [RedHairPirates, Brawler], 50, 4, 8, 300000),
        crew!("Shanks", Uncommon, [RedHairPirates, Swordsman], 20, 3, 5, 200000),
        crew!("Yasopp", Rare, [RedHairPirates, Sniper], 35, 4, 8, 300000),

        // Whitebeard Pirates
        crew!("Ace", Legendary, [WhitebearedPirates, Logia], 55, 5, 12, 500000),
        crew!("Edward Newgate", Common, [WhitebearedPirates, Emperor], 100, 8, 10, 600000),
        crew!("Jozu", Rare, [WhitebearedPirates, Paramecia], 70, 4, 8, 400000),
        crew!("Marco", Legendary, [WhitebearedPirates, Zoan], 65, 4, 12, 500000),
        crew!("Vista", Rare, [WhitebearedPirates, Swordsman], 50, 5, 8, 400000),

        // Blackbeard Pirates
        crew!("Blackbeard", Legendary, [BlackbearedPirates, Logia, Paramecia], 60, 5, 12, 500000),
        crew!("Burgess", Uncommon, [BlackbearedPirates, Brawler], 35, 2, 5, 100000),
        crew!("Devon", Rare, [BlackbearedPirates, Zoan], 40, 3, 8, 400000),
        crew!("DocQ", Rare, [BlackbearedPirates, Paramecia], 30, 2, 8, 300000),
        crew!("Kuzan", Legendary, [BlackbearedPirates, Logia], 55, 4, 12, 500000),
        crew!("Laffitte", Rare, [BlackbearedPirates], 35, 3, 8, 300000),
        crew!("Pizarro", Rare, [BlackbearedPirates], 50, 3, 8, 400000),
        crew!("Shiryu", Rare, [BlackbearedPirates, Swordsman], 50, 4, 8, 400000),
        crew!("Shot", Rare, [BlackbearedPirates], 40, 3, 8, 300000),
        crew!("VanAugur", Rare, [BlackbearedPirates, Sniper], 30, 3, 8, 400000),
        crew!("Wolf", Rare, [BlackbearedPirates, Giants], 75, 4, 9, 400000),

        // Big Mom Pirates
        crew!("Charlotte Linlin", Common, [BigMomPirates, Emperor], 110, 9, 11, 600000),
        crew!("Cracker", Rare, [BigMomPirates, Swordsman, Paramecia], 45, 5, 8, 400000),
        crew!("Katakuri", Common, [BigMomPirates, Paramecia], 80, 7, 8, 600000),
        crew!("Perospero", Rare, [BigMomPirates, Paramecia], 45, 4, 8, 400000),
        crew!("Smoothie", Legendary, [BigMomPirates, Swordsman], 55, 5, 12, 500000),

        // Heart Pirates
        crew!("Bepo", Uncommon, [HeartPirates, Brawler], 30, 2, 5, 100000),
        crew!("Jean Bart", Rare, [HeartPirates, Swordsman], 40, 3, 8, 200000),
        crew!("Law", Legendary, [HeartPirates, Swordsman, Paramecia], 60, 5, 12, 500000),
        crew!("Penguin", Uncommon, [HeartPirates], 25, 2, 5, 100000),

        // Cross Guild
        crew!("Buggy", Uncommon, [CrossGuildPirates, Paramecia], 30, 2, 5, 100000),
        crew!("Crocodile", Rare, [CrossGuildPirates, Logia], 45, 5, 8, 400000),
        crew!("Mihawk", Legendary, [CrossGuildPirates, Swordsman], 60, 6, 12, 500000),

        // Revolutionary Army
        crew!("Dragon", Legendary, [Revolutionary], 55, 4, 12, 500000),
        crew!("Sabo", Rare, [Revolutionary, Logia], 45, 3, 8, 400000),
        crew!("Emporio Ivankov", Common, [Revolutionary, Brawler], 74, 12, 10, 100000),
        crew!("Bartholomew Kuma", Rare, [Revolutionary, Paramecia, Brawler], 80, 22, 18, 300000),

        // Giants
        crew!("Brogy", Rare, [Giants, Brawler], 38, 3, 8, 300000),
        crew!("Dorry", Rare, [Giants, Swordsman], 35, 4, 8, 300000),
        crew!("Harald", Legendary, [Giants], 80, 5, 13, 500000),
        crew!("Loki", Rare, [Giants], 70, 4, 8, 400000),
        crew!("Hajrudin", Uncommon, [Giants], 30, 4, 8, 100000),

        // Gorosei (Five Elders)
        crew!("JuPeter", Legendary, [Gorosei, Zoan], 75, 6, 12, 500000),
        crew!("Mars", Legendary, [Gorosei, Zoan], 65, 7, 12, 500000),
        crew!("Nusjuro", Legendary, [Gorosei, Zoan, Swordsman], 60, 9, 12, 500000),
        crew!("Saturn", Legendary, [Gorosei, Zoan], 70, 6, 12, 500000),
        crew!("Warcury", Legendary, [Gorosei, Zoan], 80, 8, 13, 500000),

        // Holy Knights
        crew!("Figarland Garling", Legendary, [HolyKnights], 96, 26, 15, 50000),
        crew!("Figarland Shamrock", Epic, [HolyKnights], 86, 22, 12, 50000),
        crew!("Hanmayer Gunko", Rare, [HolyKnights], 73, 18, 10, 300000),
        crew!("Shepherd Sommers", Rare, [HolyKnights], 75, 19, 9, 300000),
        crew!("Rimoshifu Kiilingham", Rare, [HolyKnights], 71, 17, 9, 300000),
        crew!("Satcheis Maffey", Rare, [HolyKnights], 69, 16, 11, 300000),
    ];

    // Insert all crew templates into the database
    for template in crew_templates {
        ctx.db.crew_template().insert(template);
    }

    log::info!("Crew template database initialized with {} templates", ctx.db.crew_template().count());
}
