// Barely Modified from the Original C# code. None of the code has been modified to be idiomatic
// Rust, but rather the most direct translation possible that still compiles.

extern crate gilded_rose;

use gilded_rose::goblin::Item;

fn main() {}

fn update_aged_brie(item: &Item) -> Item {
    let mut item = item.clone();
    item.sell_in = item.sell_in - 1;

    if item.quality == 50 { return item; }
    item.quality = item.quality + 1;

    return item;
}

fn update_sulfuras(item: &Item) -> Item { return item.clone(); }

fn update_backstage_passes(item: &Item) -> Item {
    let mut item = item.clone();
    item.sell_in = item.sell_in - 1;

    if item.quality == 50 { return item; }

    item.quality = item.quality + 1;
    if item.sell_in < 10 { item.quality = item.quality + 1 }
    if item.sell_in < 5 { item.quality = item.quality + 1 }
    if item.sell_in < 0 { item.quality = 0 }

    return item;
}

fn update_normal(item: &Item) -> Item {
    let mut item = item.clone();
    item.sell_in = item.sell_in - 1;

    if item.quality == 0 { return item; }

    item.quality = item.quality - 1;
    if item.sell_in <= 0 { item.quality = item.quality - 1 }

    return item;
}

fn update_conjured_mana_cake(item: &Item) -> Item {
    let mut item = item.clone();
    item.sell_in = item.sell_in - 1;

    if item.quality == 0 { return item; }

    item.quality = item.quality - 2;
    if item.sell_in <= 0 { item.quality = item.quality - 2 }

    return item;
}

pub fn update_quality(items: Vec<Item>) -> Vec<Item> {
    items.iter().map(|i| update_item(i)).collect()
}

fn update_item(item: &Item) -> Item {
    match item.name {
        "Aged Brie" => update_aged_brie(&item),
        "Sulfuras, Hand of Ragnaros" =>  update_sulfuras(&item),
        "Backstage passes to a TAFKAL80ETC concert" => update_backstage_passes(&item),
        "Conjured Mana Cake" => update_conjured_mana_cake(&item),
        _ => update_normal(&item),
    }
}

#[test]
fn normal_items_decrease_in_quality_and_get_closer_to_sell_in_0() {
    let items = vec![
        Item { name: "Normal Item", sell_in: 10, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].sell_in, 9);
    assert_eq!(updated_items[0].quality, 19);
}

#[test]
fn normal_items_decrease_quality_twice_as_fast_after_sell_in_date_passed() {
    let items = vec![
        Item { name: "Normal Item", sell_in: 0, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 18);
}

#[test]
fn normal_items_quality_can_never_be_negative() {
    let items = vec![
        Item { name: "Normal Item", sell_in: 0, quality: 0 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 0);
}

#[test]
fn aged_brie_gets_closer_to_sell_in_0() {
    let items = vec![
        Item { name: "Aged Brie", sell_in: 10, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].sell_in, 9);
}

#[test]
fn aged_brie_increases_in_quality_as_it_gets_older() {
    let items = vec![
        Item { name: "Aged Brie", sell_in: 10, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 21);
}

#[test]
fn aged_brie_cannot_increase_past_quality_of_50() {
    let items = vec![
        Item { name: "Aged Brie", sell_in: 10, quality: 50 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 50);
}

#[test]
fn sulfuras_does_not_decrease_in_quality_or_sell_in() {
    let items = vec![
        Item { name: "Sulfuras, Hand of Ragnaros", sell_in: 10, quality: 50 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].sell_in, 10);
    assert_eq!(updated_items[0].quality, 50);
}

#[test]
fn backstage_passes_gets_closer_to_sell_in_0() {
    let items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 10, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].sell_in, 9);
}

#[test]
fn backstage_passes_increase_in_quality_by_1_above_10_days_sell_in() {
    let items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 11, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 21);
}

#[test]
fn backstage_passes_increase_in_quality_by_2_below_10_days_sell_in() {
    let items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 10, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 22);
}

#[test]
fn backstage_passes_increase_in_quality_by_3_below_5_days_sell_in() {
    let items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 5, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 23);
}

#[test]
fn backstage_passes_become_0_quality_when_sell_in_passes() {
    let items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 0, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 0);
}

#[test]
fn backstage_passes_cannot_increase_past_quality_of_50() {
    let items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 10, quality: 50 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 50);
}

#[test]
fn conjured_mana_cake_gets_closer_to_sell_by_date() {
    let items = vec![
        Item { name: "Conjured Mana Cake", sell_in: 10, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].sell_in, 9);
}

#[test]
fn conjured_mana_cake_degrades_in_quality_increments_of_2() {
    let items = vec![
        Item { name: "Conjured Mana Cake", sell_in: 10, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 18);
}

#[test]
fn conjured_mana_cakes_decrease_quality_twice_as_fast_after_sell_in_date_passed() {
    let items = vec![
        Item { name: "Conjured Mana Cake", sell_in: 0, quality: 20 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 16);
}

#[test]
fn conjured_mana_cake_quality_can_never_be_negative() {
    let items = vec![
        Item { name: "Conjured Mana Cake", sell_in: 0, quality: 0 },
    ];
    let updated_items = update_quality(items);
    assert_eq!(updated_items[0].quality, 0);
}