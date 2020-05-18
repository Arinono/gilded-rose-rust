// Barely Modified from the Original C# code. None of the code has been modified to be idiomatic
// Rust, but rather the most direct translation possible that still compiles.

extern crate gilded_rose;

use gilded_rose::goblin::Item;

fn main() {
    let mut items = vec!
    {
        Item { name: "+5 Dexterity Vest", sell_in: 10, quality: 20 },
        Item { name: "Aged Brie", sell_in: 2, quality: 0 },
        Item { name: "Elixir of the Mongoose", sell_in: 5, quality: 7 },
        Item { name: "Sulfuras, Hand of Ragnaros", sell_in: 0, quality: 80 },
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 15, quality: 20 },
        Item { name: "Conjured Mana Cake", sell_in: 3, quality: 6 }
    };

    for i in 0..50 {
        println!("Day {}:\n========================================", i);
        for item in &items {
            println!("{:?}", item);
        }
        UpdateQuality(&mut items[..]);
    }
}

fn UpdateAgedBrie(item: &mut Item) {
    item.sell_in = item.sell_in - 1;

    if item.quality == 50 { return }
    item.quality = item.quality + 1;
}

fn UpdateSulfuras(item: &mut Item) {}

fn UpdateBackstagePasses(item: &mut Item) {
    item.sell_in = item.sell_in - 1;

    if item.quality == 50 { return }

    item.quality = item.quality + 1;
    if item.sell_in < 10 { item.quality = item.quality + 1 }
    if item.sell_in < 5 { item.quality = item.quality + 1 }
    if item.sell_in < 0 { item.quality = 0 }
}

fn UpdateNormal(item: &mut Item) {
    item.sell_in = item.sell_in - 1;

    if item.quality == 0 { return }

    item.quality = item.quality - 1;
    if item.sell_in <= 0 { item.quality = item.quality - 1 }
}

fn UpdateQuality(items: &mut [Item]) {
    for i in 0..items.len() {
        UpdateItem(&mut items[i]);
    }
}

fn UpdateItem(item: &mut Item) {
    match item.name {
        "Aged Brie" => UpdateAgedBrie(item),
        "Sulfuras, Hand of Ragnaros" =>  UpdateSulfuras(item),
        "Backstage passes to a TAFKAL80ETC concert" => UpdateBackstagePasses(item),
        _ => UpdateNormal(item),
    }
}

#[test]
fn normal_items_decrease_in_quality_and_get_closer_to_sell_in_0() {
    let mut items = vec![
        Item { name: "Normal Item", sell_in: 10, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].sell_in, 9);
    assert_eq!(items[0].quality, 19);
}

#[test]
fn normal_items_decrease_quality_twice_as_fast_after_sell_in_date_passed() {
    let mut items = vec![
        Item { name: "Normal Item", sell_in: 0, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 18);
}

#[test]
fn normal_items_quality_can_never_be_negative() {
    let mut items = vec![
        Item { name: "Normal Item", sell_in: 0, quality: 0 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 0);
}

#[test]
fn aged_brie_gets_closer_to_sell_in_0() {
    let mut items = vec![
        Item { name: "Aged Brie", sell_in: 10, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].sell_in, 9);
}

#[test]
fn aged_brie_increases_in_quality_as_it_gets_older() {
    let mut items = vec![
        Item { name: "Aged Brie", sell_in: 10, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 21);
}

#[test]
fn aged_brie_cannot_increase_past_quality_of_50() {
    let mut items = vec![
        Item { name: "Aged Brie", sell_in: 10, quality: 50 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 50);
}

#[test]
fn sulfuras_does_not_decrease_in_quality_or_sell_in() {
    let mut items = vec![
        Item { name: "Sulfuras, Hand of Ragnaros", sell_in: 10, quality: 50 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].sell_in, 10);
    assert_eq!(items[0].quality, 50);
}

#[test]
fn backstage_passes_gets_closer_to_sell_in_0() {
    let mut items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 10, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].sell_in, 9);
}

#[test]
fn backstage_passes_increase_in_quality_by_1_above_10_days_sell_in() {
    let mut items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 11, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 21);
}

#[test]
fn backstage_passes_increase_in_quality_by_2_below_10_days_sell_in() {
    let mut items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 10, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 22);
}

#[test]
fn backstage_passes_increase_in_quality_by_3_below_5_days_sell_in() {
    let mut items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 5, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 23);
}

#[test]
fn backstage_passes_become_0_quality_when_sell_in_passes() {
    let mut items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 0, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 0);
}

#[test]
fn backstage_passes_cannot_increase_past_quality_of_50() {
    let mut items = vec![
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 10, quality: 50 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].quality, 50);
}