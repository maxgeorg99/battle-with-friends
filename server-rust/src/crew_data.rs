use spacetimedb::{ReducerContext, Table, log};
use crate::types::*;
use crate::tables::{CrewTemplate, crew_template};

/// Initialize the crew template database - called once on server initialization
pub fn init_crew_templates(ctx: &ReducerContext) {
    // Only initialize if the table is empty
    if ctx.db.crew_template().count() > 0 {
        log::info!("Crew templates already initialized, skipping...");
        return;
    }

    log::info!("Initializing crew template database...");

    // Crew templates (name, rarity, trait1, trait2, hp, attack, defense, cost in berries)
    let crew_templates: Vec<(&str, CrewRarity, CrewTrait, Option<CrewTrait>, u32, u32, u32, u32)> = vec![
        // Straw Hats
        ("Roronoa Zoro", CrewRarity::Rare, CrewTrait::StrawHat, Some(CrewTrait::Sword), 80, 20, 8, 300000),
        ("Nami", CrewRarity::Common, CrewTrait::StrawHat, None, 50, 12, 6, 200000),
        ("Usopp", CrewRarity::Common, CrewTrait::StrawHat, None, 60, 15, 5, 200000),
        ("Sanji", CrewRarity::Rare, CrewTrait::StrawHat, None, 75, 18, 10, 300000),
        ("Tony Tony Chopper", CrewRarity::Common, CrewTrait::StrawHat, Some(CrewTrait::Zoan), 55, 14, 7, 200000),
        ("Nico Robin", CrewRarity::Rare, CrewTrait::StrawHat, Some(CrewTrait::Paramecia), 70, 16, 9, 300000),
        ("Franky", CrewRarity::Rare, CrewTrait::StrawHat, None, 78, 19, 6, 300000),
        ("Brook", CrewRarity::Rare, CrewTrait::StrawHat, Some(CrewTrait::Paramecia), 72, 17, 8, 300000),
        ("Jinbe", CrewRarity::Epic, CrewTrait::StrawHat, None, 85, 22, 12, 500000),

        // Marines
        ("Monkey D. Garp", CrewRarity::Legendary, CrewTrait::Marine, None, 95, 25, 15, 800000),
        ("Sengoku", CrewRarity::Legendary, CrewTrait::Marine, Some(CrewTrait::Zoan), 92, 24, 14, 800000),
        ("Akainu", CrewRarity::Legendary, CrewTrait::Marine, Some(CrewTrait::Logia), 98, 26, 16, 1000000),
        ("Kizaru", CrewRarity::Legendary, CrewTrait::Marine, Some(CrewTrait::Logia), 94, 25, 14, 900000),
        ("Fujitora", CrewRarity::Epic, CrewTrait::Marine, Some(CrewTrait::Paramecia), 88, 23, 13, 600000),
        ("Smoker", CrewRarity::Rare, CrewTrait::Marine, Some(CrewTrait::Logia), 76, 19, 9, 350000),
        ("Tashigi", CrewRarity::Common, CrewTrait::Marine, Some(CrewTrait::Sword), 58, 14, 6, 200000),
        ("Koby", CrewRarity::Rare, CrewTrait::Marine, None, 68, 16, 8, 280000),

        // Revolutionary Army
        ("Monkey D. Dragon", CrewRarity::Legendary, CrewTrait::Revolutionary, None, 100, 28, 18, 1200000),
        ("Sabo", CrewRarity::Epic, CrewTrait::Revolutionary, Some(CrewTrait::Logia), 89, 23, 13, 650000),
        ("Emporio Ivankov", CrewRarity::Rare, CrewTrait::Revolutionary, Some(CrewTrait::Paramecia), 74, 18, 10, 320000),
        ("Bartholomew Kuma", CrewRarity::Epic, CrewTrait::Revolutionary, Some(CrewTrait::Paramecia), 87, 22, 12, 600000),
        ("Inazuma", CrewRarity::Common, CrewTrait::Revolutionary, Some(CrewTrait::Paramecia), 62, 15, 7, 220000),

        // Red Hair Pirates
        ("Shanks", CrewRarity::Legendary, CrewTrait::RedHairPirates, None, 99, 27, 17, 1100000),
        ("Ben Beckman", CrewRarity::Epic, CrewTrait::RedHairPirates, None, 90, 24, 14, 700000),
        ("Yasopp", CrewRarity::Rare, CrewTrait::RedHairPirates, None, 77, 20, 9, 350000),
        ("Lucky Roux", CrewRarity::Rare, CrewTrait::RedHairPirates, None, 75, 19, 11, 340000),

        // Giants
        ("Dorry", CrewRarity::Rare, CrewTrait::Giants, None, 82, 21, 8, 380000),
        ("Brogy", CrewRarity::Rare, CrewTrait::Giants, None, 82, 21, 8, 380000),
        ("Oimo", CrewRarity::Common, CrewTrait::Giants, None, 65, 16, 6, 240000),
        ("Kashii", CrewRarity::Common, CrewTrait::Giants, None, 65, 16, 6, 240000),
        ("Hajrudin", CrewRarity::Rare, CrewTrait::Giants, None, 79, 20, 9, 360000),
        ("Loki", CrewRarity::Epic, CrewTrait::Giants, Some(CrewTrait::Zoan), 79, 20, 9, 360000),

        // Five Elders
        ("St. Jaygarcia Saturn", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 97, 27, 16, 1050000),
        ("St. Marcus Mars", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 97, 27, 16, 1050000),
        ("St. Topman Warcury", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 98, 28, 17, 1100000),
        ("St. Ethanbaron V. Nusjuro", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 97, 27, 16, 1050000),
        ("St. Shepherd Ju Peter", CrewRarity::Legendary, CrewTrait::FiveElders, Some(CrewTrait::Zoan), 97, 27, 16, 1050000),

        // Holy Knights
        ("Figarland Garling", CrewRarity::Legendary, CrewTrait::HolyKnights, None, 96, 26, 15, 950000),
        ("Figarland Shamrock", CrewRarity::Epic, CrewTrait::HolyKnights, None, 86, 22, 12, 580000),
        ("Hanmayer Gunko", CrewRarity::Rare, CrewTrait::HolyKnights, None, 73, 18, 10, 330000),
        ("Shepherd Sommers", CrewRarity::Rare, CrewTrait::HolyKnights, None, 75, 19, 9, 340000),
        ("Rimoshifu Kiilingham", CrewRarity::Rare, CrewTrait::HolyKnights, None, 71, 17, 9, 310000),
        ("Satcheis Maffey", CrewRarity::Rare, CrewTrait::HolyKnights, None, 69, 16, 11, 300000),
    ];

    // Insert all crew templates into the database
    for template in crew_templates {
        ctx.db.crew_template().insert(CrewTemplate {
            id: 0,
            name: template.0.to_string(),
            rarity: template.1,
            trait1: template.2,
            trait2: template.3,
            max_hp: template.4,
            attack: template.5,
            defense: template.6,
            cost: template.7,
        });
    }

    log::info!("Crew template database initialized with {} templates", ctx.db.crew_template().count());
}
