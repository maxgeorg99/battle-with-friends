use crate::types::*;

/// Helper function to check item combinations
pub fn try_combine_items(item1: ItemComponent, item2: ItemComponent) -> Option<CompletedItem> {
    use ItemComponent::*;
    use CompletedItem::*;

    match (item1, item2) {
        // Damage / AD Focus
        (Cutlass, SniperGoggles) | (SniperGoggles, Cutlass) => Some(Yoru),
        (Cutlass, ShellDial) | (ShellDial, Cutlass) => Some(Kabuto),
        (Cutlass, SeastoneFragment) | (SeastoneFragment, Cutlass) => Some(Shusui),

        // AP Focus
        (ToneDial, ToneDial) => Some(ClimaTact),
        (ToneDial, ShellDial) | (ShellDial, ToneDial) => Some(ThunderTempo),
        (ToneDial, EnergyDrink) | (EnergyDrink, ToneDial) => Some(MirageFlower),

        // Tank Focus
        (SeastoneFragment, SeastoneFragment) => Some(AdamWood),
        (SeastoneFragment, TidalCloak) | (TidalCloak, SeastoneFragment) => Some(SeaKingScale),
        (SeastoneFragment, Meat) | (Meat, SeastoneFragment) => Some(ThousandSunnyHull),

        // Utility Focus
        (EnergyDrink, TidalCloak) | (TidalCloak, EnergyDrink) => Some(VivrCard),
        (SniperGoggles, EnergyDrink) | (EnergyDrink, SniperGoggles) => Some(LogPose),
        (ToneDial, SeastoneFragment) | (SeastoneFragment, ToneDial) => Some(Poneglyph),

        // Hybrid
        (Cutlass, Meat) | (Meat, Cutlass) => Some(GumGumFruit),
        (Meat, Meat) => Some(GomuGomuNoMi),
        (ToneDial, Cutlass) | (Cutlass, ToneDial) => Some(HakiMastery),

        _ => None,
    }
}
