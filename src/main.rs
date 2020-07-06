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
        match i.name {
            ProductTypes::AgedBrie => {
                if i.sell_in <= 0 {
                    i.quality += 2
                } else {
                    i.quality += 1
                }
                if i.quality >= 50 {
                    i.quality = 50
                }
            },
            ProductTypes::BackstagePasses => {
                if i.sell_in <= 0 {
                    i.quality = 0
                } else if i.sell_in > 0 && i.sell_in <= 5 {
                    i.quality += 3
                } else if i.sell_in > 5 && i.sell_in <= 10 {
                    i.quality += 2
                } else {
                    i.quality += 1
                }
                if i.quality >= 50 {
                    i.quality = 50
                }
            },
            ProductTypes::SulfurasHandOfRagnaros => (),
            _ => {
                if i.sell_in <= 0 {
                    i.quality -= 2
                } else {
                    i.quality -= 1
                }
                if i.quality <= 0 {
                    i.quality = 0
                }
            }
        }
        if i.name != ProductTypes::SulfurasHandOfRagnaros {
            i.sell_in -= 1
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
