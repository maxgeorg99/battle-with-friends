use spacetimedb::{ReducerContext, Table, log};
use crate::types::*;
use crate::tables::*;

/// Initialize item component stats - called once on server initialization
pub fn init_item_component_stats(ctx: &ReducerContext) {
    if ctx.db.item_component_stats().count() > 0 {
        log::info!("Item component stats already initialized, skipping...");
        return;
    }

    log::info!("Initializing item component stats...");

    let stats = vec![
        ItemComponentStats {
            component: ItemComponent::Cutlass,
            name: "Cutlass".to_string(),
            description: "+AD (Attack Damage)".to_string(),
            bonus_ad: 10,
            bonus_crit_chance: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
        },
        ItemComponentStats {
            component: ItemComponent::SniperGoggles,
            name: "Sniper Goggles".to_string(),
            description: "+Crit Chance".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.15,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
        },
        ItemComponentStats {
            component: ItemComponent::ShellDial,
            name: "Shell Dial".to_string(),
            description: "+AS (Attack Speed)".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_attack_speed: 0.15,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
        },
        ItemComponentStats {
            component: ItemComponent::ToneDial,
            name: "Tone Dial".to_string(),
            description: "+AP (Ability Power)".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 15,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
        },
        ItemComponentStats {
            component: ItemComponent::SeastoneFragment,
            name: "Seastone Fragment".to_string(),
            description: "+Armor".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 10,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
        },
        ItemComponentStats {
            component: ItemComponent::TidalCloak,
            name: "Tidal Cloak".to_string(),
            description: "+MR (Magic Resist)".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 10,
            bonus_mana: 0,
            bonus_hp: 0,
        },
        ItemComponentStats {
            component: ItemComponent::EnergyDrink,
            name: "Energy Drink".to_string(),
            description: "+Starting Mana".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 20,
            bonus_hp: 0,
        },
        ItemComponentStats {
            component: ItemComponent::Meat,
            name: "Meat".to_string(),
            description: "+HP".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 50,
        },
    ];

    for stat in stats {
        ctx.db.item_component_stats().insert(stat);
    }

    log::info!("Item component stats initialized with {} components", ctx.db.item_component_stats().count());
}

/// Initialize item combination recipes - called once on server initialization
pub fn init_item_combination_recipes(ctx: &ReducerContext) {
    if ctx.db.item_combination_recipe().count() > 0 {
        log::info!("Item combination recipes already initialized, skipping...");
        return;
    }

    log::info!("Initializing item combination recipes...");

    use ItemComponent::*;
    use CompletedItem::*;

    let recipes = vec![
        // Damage / AD Focus
        ItemCombinationRecipe { id: 0, component1: Cutlass, component2: SniperGoggles, result: Yoru },
        ItemCombinationRecipe { id: 0, component1: Cutlass, component2: ShellDial, result: Kabuto },
        ItemCombinationRecipe { id: 0, component1: Cutlass, component2: SeastoneFragment, result: Shusui },

        // AP Focus
        ItemCombinationRecipe { id: 0, component1: ToneDial, component2: ToneDial, result: ClimaTact },
        ItemCombinationRecipe { id: 0, component1: ToneDial, component2: ShellDial, result: ThunderTempo },
        ItemCombinationRecipe { id: 0, component1: ToneDial, component2: EnergyDrink, result: MirageFlower },

        // Tank Focus
        ItemCombinationRecipe { id: 0, component1: SeastoneFragment, component2: SeastoneFragment, result: AdamWood },
        ItemCombinationRecipe { id: 0, component1: SeastoneFragment, component2: TidalCloak, result: SeaKingScale },
        ItemCombinationRecipe { id: 0, component1: SeastoneFragment, component2: Meat, result: ThousandSunnyHull },

        // Utility Focus
        ItemCombinationRecipe { id: 0, component1: EnergyDrink, component2: TidalCloak, result: VivrCard },
        ItemCombinationRecipe { id: 0, component1: SniperGoggles, component2: EnergyDrink, result: LogPose },
        ItemCombinationRecipe { id: 0, component1: ToneDial, component2: SeastoneFragment, result: Poneglyph },

        // Hybrid
        ItemCombinationRecipe { id: 0, component1: Cutlass, component2: Meat, result: GumGumFruit },
        ItemCombinationRecipe { id: 0, component1: Meat, component2: Meat, result: GomuGomuNoMi },
        ItemCombinationRecipe { id: 0, component1: ToneDial, component2: Cutlass, result: HakiMastery },
    ];

    for recipe in recipes {
        ctx.db.item_combination_recipe().insert(recipe);
    }

    log::info!("Item combination recipes initialized with {} recipes", ctx.db.item_combination_recipe().count());
}

/// Initialize completed item stats - called once on server initialization
pub fn init_completed_item_stats(ctx: &ReducerContext) {
    if ctx.db.completed_item_stats().count() > 0 {
        log::info!("Completed item stats already initialized, skipping...");
        return;
    }

    log::info!("Initializing completed item stats...");

    let stats = vec![
        // Damage / AD Focus
        CompletedItemStats {
            item: CompletedItem::Yoru,
            name: "Yoru".to_string(),
            description: "Cutlass + Sniper Goggles: +75% crit damage".to_string(),
            bonus_ad: 10,
            bonus_crit_chance: 0.15,
            bonus_crit_damage: 0.75,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::Kabuto,
            name: "Kabuto".to_string(),
            description: "Cutlass + Shell Dial: Attacks deal splash damage".to_string(),
            bonus_ad: 10,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.15,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: true,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::Shusui,
            name: "Shusui".to_string(),
            description: "Cutlass + Seastone Fragment: Bonus AD + armor shred".to_string(),
            bonus_ad: 20,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 10,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 5,
        },

        // AP Focus
        CompletedItemStats {
            item: CompletedItem::ClimaTact,
            name: "Clima-Tact".to_string(),
            description: "Tone Dial + Tone Dial: Doubles AP".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 30,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::ThunderTempo,
            name: "Thunder Tempo".to_string(),
            description: "Tone Dial + Shell Dial: AP + attack speed".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.15,
            bonus_ap: 15,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::MirageFlower,
            name: "Mirage Flower".to_string(),
            description: "Tone Dial + Energy Drink: AP + starting mana".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 15,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 20,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },

        // Tank Focus
        CompletedItemStats {
            item: CompletedItem::AdamWood,
            name: "Adam Wood".to_string(),
            description: "Seastone Fragment + Seastone Fragment: Massive armor".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 30,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::SeaKingScale,
            name: "Sea King Scale".to_string(),
            description: "Seastone Fragment + Tidal Cloak: Armor + MR".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 10,
            bonus_mr: 10,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::ThousandSunnyHull,
            name: "Thousand Sunny Hull".to_string(),
            description: "Seastone Fragment + Meat: Armor + HP".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 10,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 50,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },

        // Utility Focus
        CompletedItemStats {
            item: CompletedItem::VivrCard,
            name: "Vivre Card".to_string(),
            description: "Energy Drink + Tidal Cloak: Mana + survivability".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 10,
            bonus_mana: 20,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::LogPose,
            name: "Log Pose".to_string(),
            description: "Sniper Goggles + Energy Drink: Crit + mana".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.15,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 20,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::Poneglyph,
            name: "Poneglyph".to_string(),
            description: "Tone Dial + Seastone Fragment: AP + armor".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 15,
            bonus_armor: 10,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },

        // Hybrid
        CompletedItemStats {
            item: CompletedItem::GumGumFruit,
            name: "Gum-Gum Fruit".to_string(),
            description: "Cutlass + Meat: AD + HP".to_string(),
            bonus_ad: 10,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 50,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::GomuGomuNoMi,
            name: "Gomu Gomu no Mi".to_string(),
            description: "Meat + Meat: Massive HP regeneration".to_string(),
            bonus_ad: 0,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 0,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 100,
            bonus_hp_regen: 5.0,
            has_splash: false,
            armor_shred: 0,
        },
        CompletedItemStats {
            item: CompletedItem::HakiMastery,
            name: "Haki Mastery".to_string(),
            description: "Tone Dial + Cutlass: AD + AP hybrid".to_string(),
            bonus_ad: 10,
            bonus_crit_chance: 0.0,
            bonus_crit_damage: 0.0,
            bonus_attack_speed: 0.0,
            bonus_ap: 15,
            bonus_armor: 0,
            bonus_mr: 0,
            bonus_mana: 0,
            bonus_hp: 0,
            bonus_hp_regen: 0.0,
            has_splash: false,
            armor_shred: 0,
        },
    ];

    for stat in stats {
        ctx.db.completed_item_stats().insert(stat);
    }

    log::info!("Completed item stats initialized with {} items", ctx.db.completed_item_stats().count());
}

/// Helper function to check item combinations using the recipe table
pub fn try_combine_items(ctx: &ReducerContext, item1: ItemComponent, item2: ItemComponent) -> Option<CompletedItem> {
    // Check both orderings (item1 + item2 and item2 + item1)
    for recipe in ctx.db.item_combination_recipe().iter() {
        if (recipe.component1 == item1 && recipe.component2 == item2) ||
           (recipe.component1 == item2 && recipe.component2 == item1) {
            return Some(recipe.result);
        }
    }
    None
}
