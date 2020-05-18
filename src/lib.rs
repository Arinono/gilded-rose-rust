pub mod goblin;

use goblin::Item;
use std::collections::hash_map::HashMap;

mod update;
mod cond;

use cond::*;
use update::*;

enum ItemBehavior {
    EachDay {
        condition: Box<dyn ItemCondition>,
        effects: Vec<ItemChange>,
    },
    All(Vec<ItemBehavior>),
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
        ItemBehavior::All(list) => {
            list.iter()
                .map(|beh| update_item_each_day(&mut item, beh))
                .count();
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

struct StoreSettings {
    item_behaviors: HashMap<String, ItemBehavior>,
    default_behavior: ItemBehavior,
}

impl StoreSettings {
    fn new(default_behavior: ItemBehavior) -> Self {
        StoreSettings {
            item_behaviors: HashMap::new(),
            default_behavior,
        }
    }

    fn add_item_behaviors(&mut self, item_id: String, behaviors: Vec<ItemBehavior>) {
        self.item_behaviors
            .insert(item_id, ItemBehavior::All(behaviors));
    }
}

use test_store::new_test_store;
mod test_store {
    use super::*;

    pub(super) fn new_test_store() -> StoreSettings {
        let mut settings = StoreSettings::new(ItemBehavior::All(vec![
            ItemBehavior::each_day_decrease_sell_in(),
            ItemBehavior::EachDay {
                condition: Box::new(AlwaysTrueCondition),
                effects: vec![ItemChange::DecreaseQuality(1)],
            },
            ItemBehavior::EachDay {
                condition: Box::new(SellByCondition::After),
                effects: vec![ItemChange::DecreaseQuality(1)],
            },
        ]));

        settings.add_item_behaviors(
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
        settings.add_item_behaviors(String::from("Sulfuras, Hand of Ragnaros"), vec![]);
        settings.add_item_behaviors(
            String::from("Aged Brie"),
            vec![
                ItemBehavior::each_day_decrease_sell_in(),
                ItemBehavior::EachDay {
                    condition: Box::new(AlwaysTrueCondition),
                    effects: vec![ItemChange::IncreaseQuality(1)],
                },
            ],
        );
        settings.add_item_behaviors(
            String::from("Backstage passes to a TAFKAL80ETC concert"),
            vec![
                ItemBehavior::each_day_decrease_sell_in(),
                ItemBehavior::EachDay {
                    condition: Box::new(AlwaysTrueCondition),
                    effects: vec![ItemChange::IncreaseQuality(1)],
                },
                ItemBehavior::EachDay {
                    condition: Box::new(SellByCondition::WithinNDaysExclusive(10)),
                    effects: vec![ItemChange::IncreaseQuality(1)],
                },
                ItemBehavior::EachDay {
                    condition: Box::new(SellByCondition::WithinNDaysExclusive(5)),
                    effects: vec![ItemChange::IncreaseQuality(1)],
                },
                ItemBehavior::EachDay {
                    condition: Box::new(SellByCondition::OnDayOrAfter),
                    effects: vec![ItemChange::SetQuality(0)],
                },
            ],
        );

        settings
    }

}

/// Called after one day passes, all items should be updated accordingly
pub fn update_quality(items: &mut Vec<Item>) {
    for mut item in items.iter_mut() {
        update_item(&mut item);
    }
}

fn update_item(mut item: &mut Item) {
    let standard = new_test_store();
    let apply_behaviors = standard
        .item_behaviors
        .get(item.name)
        .unwrap_or(&standard.default_behavior);

    // one day passes...
    update_item_each_day(&mut item, &apply_behaviors);
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
    let mut item = Item {
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
