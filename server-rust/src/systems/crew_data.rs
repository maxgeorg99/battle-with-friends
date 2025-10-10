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

    // Crew templates - using struct directly with auto-incremented IDs
    let crew_templates = vec![
        // Straw Hats
        CrewTemplate { id: 0, name: "Roronoa Zoro".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::StrawHat, trait2: Some(CrewTrait::Sword), max_hp: 80, attack: 20, defense: 8, cost: 300000 },
        CrewTemplate { id: 0, name: "Nami".to_string(), rarity: CrewRarity::Common, trait1: CrewTrait::StrawHat, trait2: None, max_hp: 50, attack: 12, defense: 6, cost: 200000 },
        CrewTemplate { id: 0, name: "Usopp".to_string(), rarity: CrewRarity::Common, trait1: CrewTrait::StrawHat, trait2: None, max_hp: 60, attack: 15, defense: 5, cost: 200000 },
        CrewTemplate { id: 0, name: "Sanji".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::StrawHat, trait2: None, max_hp: 75, attack: 18, defense: 10, cost: 300000 },
        CrewTemplate { id: 0, name: "Tony Tony Chopper".to_string(), rarity: CrewRarity::Common, trait1: CrewTrait::StrawHat, trait2: Some(CrewTrait::Zoan), max_hp: 55, attack: 14, defense: 7, cost: 200000 },
        CrewTemplate { id: 0, name: "Nico Robin".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::StrawHat, trait2: Some(CrewTrait::Paramecia), max_hp: 70, attack: 16, defense: 9, cost: 300000 },
        CrewTemplate { id: 0, name: "Franky".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::StrawHat, trait2: None, max_hp: 78, attack: 19, defense: 6, cost: 300000 },
        CrewTemplate { id: 0, name: "Brook".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::StrawHat, trait2: Some(CrewTrait::Paramecia), max_hp: 72, attack: 17, defense: 8, cost: 300000 },
        CrewTemplate { id: 0, name: "Jinbe".to_string(), rarity: CrewRarity::Epic, trait1: CrewTrait::StrawHat, trait2: None, max_hp: 85, attack: 22, defense: 12, cost: 500000 },

        // Marines
        CrewTemplate { id: 0, name: "Monkey D. Garp".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::Marine, trait2: None, max_hp: 95, attack: 25, defense: 15, cost: 800000 },
        CrewTemplate { id: 0, name: "Sengoku".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::Marine, trait2: Some(CrewTrait::Zoan), max_hp: 92, attack: 24, defense: 14, cost: 800000 },
        CrewTemplate { id: 0, name: "Akainu".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::Marine, trait2: Some(CrewTrait::Logia), max_hp: 98, attack: 26, defense: 16, cost: 1000000 },
        CrewTemplate { id: 0, name: "Kizaru".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::Marine, trait2: Some(CrewTrait::Logia), max_hp: 94, attack: 25, defense: 14, cost: 900000 },
        CrewTemplate { id: 0, name: "Fujitora".to_string(), rarity: CrewRarity::Epic, trait1: CrewTrait::Marine, trait2: Some(CrewTrait::Paramecia), max_hp: 88, attack: 23, defense: 13, cost: 600000 },
        CrewTemplate { id: 0, name: "Smoker".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::Marine, trait2: Some(CrewTrait::Logia), max_hp: 76, attack: 19, defense: 9, cost: 350000 },
        CrewTemplate { id: 0, name: "Tashigi".to_string(), rarity: CrewRarity::Common, trait1: CrewTrait::Marine, trait2: Some(CrewTrait::Sword), max_hp: 58, attack: 14, defense: 6, cost: 200000 },
        CrewTemplate { id: 0, name: "Koby".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::Marine, trait2: None, max_hp: 68, attack: 16, defense: 8, cost: 280000 },

        // Revolutionary Army
        CrewTemplate { id: 0, name: "Monkey D. Dragon".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::Revolutionary, trait2: None, max_hp: 100, attack: 28, defense: 18, cost: 1200000 },
        CrewTemplate { id: 0, name: "Sabo".to_string(), rarity: CrewRarity::Epic, trait1: CrewTrait::Revolutionary, trait2: Some(CrewTrait::Logia), max_hp: 89, attack: 23, defense: 13, cost: 650000 },
        CrewTemplate { id: 0, name: "Emporio Ivankov".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::Revolutionary, trait2: Some(CrewTrait::Paramecia), max_hp: 74, attack: 18, defense: 10, cost: 320000 },
        CrewTemplate { id: 0, name: "Bartholomew Kuma".to_string(), rarity: CrewRarity::Epic, trait1: CrewTrait::Revolutionary, trait2: Some(CrewTrait::Paramecia), max_hp: 87, attack: 22, defense: 12, cost: 600000 },
        CrewTemplate { id: 0, name: "Inazuma".to_string(), rarity: CrewRarity::Common, trait1: CrewTrait::Revolutionary, trait2: Some(CrewTrait::Paramecia), max_hp: 62, attack: 15, defense: 7, cost: 220000 },

        // Red Hair Pirates
        CrewTemplate { id: 0, name: "Shanks".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::RedHairPirates, trait2: None, max_hp: 99, attack: 27, defense: 17, cost: 1100000 },
        CrewTemplate { id: 0, name: "Ben Beckman".to_string(), rarity: CrewRarity::Epic, trait1: CrewTrait::RedHairPirates, trait2: None, max_hp: 90, attack: 24, defense: 14, cost: 700000 },
        CrewTemplate { id: 0, name: "Yasopp".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::RedHairPirates, trait2: None, max_hp: 77, attack: 20, defense: 9, cost: 350000 },
        CrewTemplate { id: 0, name: "Lucky Roux".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::RedHairPirates, trait2: None, max_hp: 75, attack: 19, defense: 11, cost: 340000 },

        // Giants
        CrewTemplate { id: 0, name: "Dorry".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::Giants, trait2: None, max_hp: 82, attack: 21, defense: 8, cost: 380000 },
        CrewTemplate { id: 0, name: "Brogy".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::Giants, trait2: None, max_hp: 82, attack: 21, defense: 8, cost: 380000 },
        CrewTemplate { id: 0, name: "Oimo".to_string(), rarity: CrewRarity::Common, trait1: CrewTrait::Giants, trait2: None, max_hp: 65, attack: 16, defense: 6, cost: 240000 },
        CrewTemplate { id: 0, name: "Kashii".to_string(), rarity: CrewRarity::Common, trait1: CrewTrait::Giants, trait2: None, max_hp: 65, attack: 16, defense: 6, cost: 240000 },
        CrewTemplate { id: 0, name: "Hajrudin".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::Giants, trait2: None, max_hp: 79, attack: 20, defense: 9, cost: 360000 },
        CrewTemplate { id: 0, name: "Loki".to_string(), rarity: CrewRarity::Epic, trait1: CrewTrait::Giants, trait2: Some(CrewTrait::Zoan), max_hp: 79, attack: 20, defense: 9, cost: 360000 },

        // Five Elders
        CrewTemplate { id: 0, name: "St. Jaygarcia Saturn".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::FiveElders, trait2: Some(CrewTrait::Zoan), max_hp: 97, attack: 27, defense: 16, cost: 1050000 },
        CrewTemplate { id: 0, name: "St. Marcus Mars".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::FiveElders, trait2: Some(CrewTrait::Zoan), max_hp: 97, attack: 27, defense: 16, cost: 1050000 },
        CrewTemplate { id: 0, name: "St. Topman Warcury".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::FiveElders, trait2: Some(CrewTrait::Zoan), max_hp: 98, attack: 28, defense: 17, cost: 1100000 },
        CrewTemplate { id: 0, name: "St. Ethanbaron V. Nusjuro".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::FiveElders, trait2: Some(CrewTrait::Zoan), max_hp: 97, attack: 27, defense: 16, cost: 1050000 },
        CrewTemplate { id: 0, name: "St. Shepherd Ju Peter".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::FiveElders, trait2: Some(CrewTrait::Zoan), max_hp: 97, attack: 27, defense: 16, cost: 1050000 },

        // Holy Knights
        CrewTemplate { id: 0, name: "Figarland Garling".to_string(), rarity: CrewRarity::Legendary, trait1: CrewTrait::HolyKnights, trait2: None, max_hp: 96, attack: 26, defense: 15, cost: 950000 },
        CrewTemplate { id: 0, name: "Figarland Shamrock".to_string(), rarity: CrewRarity::Epic, trait1: CrewTrait::HolyKnights, trait2: None, max_hp: 86, attack: 22, defense: 12, cost: 580000 },
        CrewTemplate { id: 0, name: "Hanmayer Gunko".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::HolyKnights, trait2: None, max_hp: 73, attack: 18, defense: 10, cost: 330000 },
        CrewTemplate { id: 0, name: "Shepherd Sommers".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::HolyKnights, trait2: None, max_hp: 75, attack: 19, defense: 9, cost: 340000 },
        CrewTemplate { id: 0, name: "Rimoshifu Kiilingham".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::HolyKnights, trait2: None, max_hp: 71, attack: 17, defense: 9, cost: 310000 },
        CrewTemplate { id: 0, name: "Satcheis Maffey".to_string(), rarity: CrewRarity::Rare, trait1: CrewTrait::HolyKnights, trait2: None, max_hp: 69, attack: 16, defense: 11, cost: 300000 },
    ];

    // Insert all crew templates into the database
    for template in crew_templates {
        ctx.db.crew_template().insert(template);
    }

    log::info!("Crew template database initialized with {} templates", ctx.db.crew_template().count());
}
