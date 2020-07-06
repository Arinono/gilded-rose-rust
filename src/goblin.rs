// This module cannot be changed. Other users depend ont the Item struct, so making any changes to
// it is not modifiable in the short term.
// -- The Management

/// TODO: add documentation here
/// -- The Management

use std::fmt;

#[derive(Debug)]
pub struct Item {
    pub name: ProductTypes,
    pub sell_in: i64,
    pub quality: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProductTypes {
    AgedBrie,
    BackstagePasses,
    ConjuredManaCake,
    DexterityVest,
    ElixirOfTheMongoose,
    SulfurasHandOfRagnaros,
}

impl ProductTypes {
    pub fn name(&self) -> &str {
        match self {
            Self::AgedBrie => "Aged Brie",
            Self::BackstagePasses => "Backstage passes to a TAFKAL80ETC concert",
            Self::ConjuredManaCake => "Conjured Mana Cake",
            Self::DexterityVest => "+5 Dexterity Vest",
            Self::ElixirOfTheMongoose => "Elixir of the Mongoose",
            Self::SulfurasHandOfRagnaros => "Sulfuras, Hand of Ragnaros",
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item {{ name: \"{}\", sell_in: {}, quality: {} }}", self.name.name(), self.sell_in, self.quality)
    }
}
