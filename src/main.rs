// Barely Modified from the Original C# code. None of the code has been modified to be idiomatic
// Rust, but rather the most direct translation possible that still compiles.

extern crate gilded_rose;

use gilded_rose::goblin::*;

fn main() {
    let mut items = vec![
        Item {
            name: ProductTypes::DexterityVest,
            sell_in: 10,
            quality: 20,
        },
        Item {
            name: ProductTypes::AgedBrie,
            sell_in: 2,
            quality: 0,
        },
        Item {
            name: ProductTypes::ElixirOfTheMongoose,
            sell_in: 5,
            quality: 7,
        },
        Item {
            name: ProductTypes::SulfurasHandOfRagnaros,
            sell_in: 0,
            quality: 80,
        },
        Item {
            name: ProductTypes::BackstagePasses,
            sell_in: 15,
            quality: 20,
        },
        Item {
            name: ProductTypes::ConjuredManaCake,
            sell_in: 3,
            quality: 6,
        },
    ];

    for i in 0..50 {
        println!("Day {}:\n========================================", i);
        for item in &items {
            println!("{}", format!("{}", item));
        }
        update_quality(&mut items[..]);
    }
}

fn update_quality(items: &mut [Item]) {
    for i in items {
        i.quality = _update_quality(i);
        i.quality = _normalize_quality(i);
        i.sell_in = _update_sell_in(i);
    }
}

fn _update_sell_in (i: &Item) -> i64 {
    if i.name != ProductTypes::SulfurasHandOfRagnaros {
        i.sell_in - 1
    } else {
        i.sell_in
    }
}
 
fn _normalize_quality(i: &Item) -> i64 {
    if i.name == ProductTypes::SulfurasHandOfRagnaros {
        return i.quality;
    }
    if i.quality >= 50 {
        return 50;
    } else if i.quality <= 0 {
        return 0
    } else {
        return i.quality;
    }
}

fn _update_quality(item: &Item) -> i64 {
    match item.name {
        ProductTypes::AgedBrie => {
            if item.sell_in <= 0 {
                item.quality + 2
            } else {
                item.quality + 1
            }
        },
        ProductTypes::BackstagePasses => {
            if item.sell_in <= 0 {
                item.quality - item.quality
            } else if item.sell_in > 0 && item.sell_in <= 5 {
                item.quality + 3
            } else if item.sell_in > 5 && item.sell_in <= 10 {
                item.quality + 2
            } else {
                item.quality + 1
            }
        },
        ProductTypes::SulfurasHandOfRagnaros => item.quality,
        _ => {
            if item.sell_in <= 0 {
                item.quality - 2
            } else {
                item.quality - 1
            }
        }
    }
}
#[test]
fn normal_items_decrease_quality() {
    let mut items = vec![Item {
        name: ProductTypes::DexterityVest,
        sell_in: 10,
        quality: 20,
    }];
    update_quality(&mut items[..]);
    assert_eq!(items[0].sell_in, 9);
    assert_eq!(items[0].quality, 19);
}
