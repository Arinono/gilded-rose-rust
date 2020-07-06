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
    for i in 0..items.len() {
        if items[i].name != ProductTypes::AgedBrie
            && items[i].name != ProductTypes::BackstagePasses
        {
            if items[i].quality > 0 {
                if items[i].name != ProductTypes::SulfurasHandOfRagnaros {
                    items[i].quality = items[i].quality - 1;
                }
            }
        } else {
            if items[i].quality < 50 {
                items[i].quality = items[i].quality + 1;

                if items[i].name == ProductTypes::BackstagePasses {
                    if items[i].sell_in < 11 {
                        if items[i].quality < 50 {
                            items[i].quality = items[i].quality + 1;
                        }
                    }

                    if items[i].sell_in < 6 {
                        if items[i].quality < 50 {
                            items[i].quality = items[i].quality + 1;
                        }
                    }
                }
            }
        }

        if items[i].name != ProductTypes::SulfurasHandOfRagnaros {
            items[i].sell_in = items[i].sell_in - 1;
        }

        if items[i].sell_in < 0 {
            if items[i].name != ProductTypes::AgedBrie {
                if items[i].name != ProductTypes::BackstagePasses {
                    if items[i].quality > 0 {
                        if items[i].name != ProductTypes::SulfurasHandOfRagnaros {
                            items[i].quality = items[i].quality - 1;
                        }
                    }
                } else {
                    items[i].quality = items[i].quality - items[i].quality;
                }
            } else {
                if items[i].quality < 50 {
                    items[i].quality = items[i].quality + 1;
                }
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
