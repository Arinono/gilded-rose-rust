pub mod goblin;

use goblin::Item;
use std::collections::hash_map::HashMap;

struct SellBy {
    day: usize,
}

trait ItemCondition {
    fn item_satisfies_condition(&self, item: &Item) -> bool;
}

enum SellByCondition {
    OnDayOrAfter,
    After,
    Before,
    OnDayOrBefore,
    WithinNDaysInclusive(usize),
}

impl ItemCondition for SellByCondition {
    fn item_satisfies_condition(&self, item: &Item) -> bool {
        use SellByCondition::*;
        match self {
            OnDayOrAfter => item.sell_in <= 0,
            After => item.sell_in < 0,
            Before => item.sell_in > 0,
            OnDayOrBefore => item.sell_in >= 0,
            WithinNDaysInclusive(n) => item.sell_in > 0 && item.sell_in <= (*n as i64),
        }
    }
}
struct AlwaysTrueCondition;
impl ItemCondition for AlwaysTrueCondition {
    fn item_satisfies_condition(&self, item: &Item) -> bool {
        true
    }
}

enum ItemChange {
    DecreaseQuality(usize),
    IncreaseQuality(usize),
    DecreaseSellInByOne,
    SetQuality(i64),
}

// Not sure how this will work for Sulfuras
impl ItemChange {
    fn apply_item_change(&self, item: &mut Item) {
        use ItemChange::*;
        match self {
            DecreaseQuality(amount) => item.quality -= *amount as i64,
            IncreaseQuality(amount) => item.quality += *amount as i64,
            DecreaseSellInByOne => item.sell_in -= 1,
            SetQuality(amount) => item.quality = *amount,
        }

        // universal rule quality isn't
        if item.quality < 0 {
            item.quality = 0;
        }

        if item.quality > 50 {
            item.quality = 50;
        }
    }
}

enum ItemBehavior {
    EachDay {
        condition: Box<dyn ItemCondition>,
        effects: Vec<ItemChange>,
    },
}

fn update_item_each_day(mut item: &mut Item, behavior: &ItemBehavior) {
    match behavior {
        ItemBehavior::EachDay { condition, effects } => {
            if condition.item_satisfies_condition(item) {
                for eff in effects {
                    eff.apply_item_change(&mut item);
                }
            }
        }
    }
}

impl ItemBehavior {
    pub fn each_day_decrease_sell_in() -> Self {
        ItemBehavior::EachDay {
            condition: Box::new(AlwaysTrueCondition),
            effects: vec![ItemChange::DecreaseSellInByOne],
        }
    }
}


pub struct StoreSettings {
    normal_behavior: Vec<ItemBehavior>,
    item_behaviors: HashMap<String, Vec<ItemBehavior>>,
}

use test_store::new_test_store;
mod test_store {
    use super::*;

    pub fn new_test_store() -> StoreSettings {
        let mut item_behaviors: HashMap<String, Vec<ItemBehavior>> = HashMap::new();
        item_behaviors.insert(
            String::from("Conjured Mana Cake"),
            vec![
                ItemBehavior::each_day_decrease_sell_in(),
                ItemBehavior::EachDay {
                    condition: Box::new(AlwaysTrueCondition),
                    effects: vec![ItemChange::DecreaseQuality(2)],
                },
                ItemBehavior::EachDay {
                    condition: Box::new(SellByCondition::After),
                    effects: vec![ItemChange::DecreaseQuality(2)],
                },
            ],
        );
        item_behaviors.insert(
            String::from("Sulfuras, Hand of Ragnaros"),
            vec![],
        );
        item_behaviors.insert(
            String::from("Aged Brie"),
            vec![
                ItemBehavior::each_day_decrease_sell_in(),
                ItemBehavior::EachDay {
                    condition: Box::new(AlwaysTrueCondition),
                    effects: vec![ItemChange::IncreaseQuality(1)],
                },
            ],
        );
        item_behaviors.insert(
            String::from("Backstage passes to a TAFKAL80ETC concert"),
            vec![
                ItemBehavior::each_day_decrease_sell_in(),
                ItemBehavior::EachDay {
                    condition: Box::new(AlwaysTrueCondition),
                    effects: vec![ItemChange::IncreaseQuality(1)],
                },
                ItemBehavior::EachDay {
                    condition: Box::new(SellByCondition::WithinNDaysInclusive(9)),
                    effects: vec![ItemChange::IncreaseQuality(1)],
                },
                ItemBehavior::EachDay {
                    condition: Box::new(SellByCondition::WithinNDaysInclusive(4)),
                    effects: vec![ItemChange::IncreaseQuality(1)],
                },
                ItemBehavior::EachDay {
                    condition: Box::new(SellByCondition::OnDayOrAfter),
                    effects: vec![ItemChange::SetQuality(0)],
                },
            ],
        );
        

        StoreSettings {
            normal_behavior: vec![
                ItemBehavior::each_day_decrease_sell_in(),
                ItemBehavior::EachDay {
                    condition: Box::new(AlwaysTrueCondition),
                    effects: vec![ItemChange::DecreaseQuality(1)],
                },
                ItemBehavior::EachDay {
                    condition: Box::new(SellByCondition::After),
                    effects: vec![ItemChange::DecreaseQuality(1)],
                },
            ],
            item_behaviors,
        }
    }

    fn update_backstage_passes(item: Item) -> Item {
        let mut item = item.clone();
        item.sell_in = item.sell_in - 1;

        if item.quality == 50 {
            return item;
        }

        item.quality = item.quality + 1;

        if item.sell_in < 10 {
            item.quality = item.quality + 1
        }
        if item.sell_in < 5 {
            item.quality = item.quality + 1
        }
        if item.sell_in < 0 {
            item.quality = 0
        }
        return item;
    }

    fn update_conjured_mana_cake(item: Item) -> Item {
        let mut item = item.clone();
        item.sell_in = item.sell_in - 1;

        if item.quality == 0 {
            return item;
        }

        item.quality = item.quality - 2;
        if item.sell_in <= 0 {
            item.quality = item.quality - 2
        }

        return item;
    }
}

/// Called after one day passes, all items should be updated accordingly
pub fn update_quality(mut items: &mut Vec<Item>) {
    for mut item in items.iter_mut() {
        update_item(&mut item);
    }
}

fn update_item(mut item: &mut Item) {
    let standard = new_test_store();
    let apply_behaviors = standard.item_behaviors.get(item.name).unwrap_or(&standard.normal_behavior);

    // one day passes...
    for beh in apply_behaviors.iter() {
        update_item_each_day(&mut item, &beh);
    }
    // match item.name {
    //     "Aged Brie" => update_aged_brie(Item),
    //     "Sulfuras, Hand of Ragnaros" => update_sulfuras(Item),
    //     "Backstage passes to a TAFKAL80ETC concert" => update_backstage_passes(Item),
    //     "Conjured Mana Cake" => update_conjured_mana_cake(Item),
    //     _ => update_normal(Item),
    // }
}

#[test]
fn update_item_updates_all_items() {
    let mut items = vec![
        Item {
            name: "Normal Item",
            sell_in: 10,
            quality: 20,
        },
        Item {
            name: "Conjured Mana Cake",
            sell_in: 10,
            quality: 20,
        },
    ];
    update_quality(&mut items);
    assert_eq!(items[0].sell_in, 9);
    assert_eq!(items[0].quality, 19);

    assert_eq!(items[1].sell_in, 9);
    assert_eq!(items[1].quality, 18);
}

#[test]
fn normal_items_decrease_in_quality_and_get_closer_to_sell_in_0() {
    let mut item = Item {
        name: "Normal Item",
        sell_in: 10,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.sell_in, 9);
    assert_eq!(item.quality, 19);
}

#[test]
fn normal_items_decrease_quality_twice_as_fast_after_sell_in_date_passed() {
    let mut item = Item {
        name: "Normal Item",
        sell_in: 0,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 18);
}

#[test]
fn normal_items_quality_can_never_be_negative() {
    let mut item = Item {
        name: "Normal Item",
        sell_in: 0,
        quality: 0,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 0);
}

#[test]
fn aged_brie_gets_closer_to_sell_in_0() {
    let mut item  = Item {
        name: "Aged Brie",
        sell_in: 10,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.sell_in, 9);
}

#[test]
fn aged_brie_increases_in_quality_as_it_gets_older() {
    let mut item = Item {
        name: "Aged Brie",
        sell_in: 10,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 21);
}

#[test]
fn aged_brie_cannot_increase_past_quality_of_50() {
    let mut item = Item {
        name: "Aged Brie",
        sell_in: 10,
        quality: 50,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 50);
}

#[test]
fn sulfuras_does_not_decrease_in_quality_or_sell_in() {
    let mut item = Item {
        name: "Sulfuras, Hand of Ragnaros",
        sell_in: 10,
        quality: 50,
    };
    update_item(&mut item);
    assert_eq!(item.sell_in, 10);
    assert_eq!(item.quality, 50);
}

#[test]
fn backstage_passes_gets_closer_to_sell_in_0() {
    let mut item = Item {
        name: "Backstage passes to a TAFKAL80ETC concert",
        sell_in: 10,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.sell_in, 9);
}

#[test]
fn backstage_passes_increase_in_quality_by_1_above_10_days_sell_in() {
    let mut item = Item {
        name: "Backstage passes to a TAFKAL80ETC concert",
        sell_in: 11,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 21);
}

#[test]
fn backstage_passes_increase_in_quality_by_2_below_10_days_sell_in() {
    let mut item = Item {
        name: "Backstage passes to a TAFKAL80ETC concert",
        sell_in: 10,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 22);
}

#[test]
fn backstage_passes_increase_in_quality_by_3_below_5_days_sell_in() {
    let mut item = Item {
        name: "Backstage passes to a TAFKAL80ETC concert",
        sell_in: 5,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 23);
}

#[test]
fn backstage_passes_become_0_quality_when_sell_in_passes() {
    let mut item = Item {
        name: "Backstage passes to a TAFKAL80ETC concert",
        sell_in: 0,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 0);
}

#[test]
fn backstage_passes_cannot_increase_past_quality_of_50() {
    let mut item = Item {
        name: "Backstage passes to a TAFKAL80ETC concert",
        sell_in: 10,
        quality: 50,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 50);
}

#[test]
fn conjured_mana_cake_gets_closer_to_sell_by_date() {
    let mut item = Item {
        name: "Conjured Mana Cake",
        sell_in: 10,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.sell_in, 9);
}

#[test]
fn conjured_mana_cake_degrades_in_quality_increments_of_2() {
    let mut item = Item {
        name: "Conjured Mana Cake",
        sell_in: 10,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 18);
}

#[test]
fn conjured_mana_cakes_decrease_quality_twice_as_fast_after_sell_in_date_passed() {
    let mut item = Item {
        name: "Conjured Mana Cake",
        sell_in: 0,
        quality: 20,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 16);
}

#[test]
fn conjured_mana_cake_quality_can_never_be_negative() {
    let mut item = Item {
        name: "Conjured Mana Cake",
        sell_in: 0,
        quality: 0,
    };
    update_item(&mut item);
    assert_eq!(item.quality, 0);
}
