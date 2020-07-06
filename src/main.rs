// Barely Modified from the Original C# code. None of the code has been modified to be idiomatic
// Rust, but rather the most direct translation possible that still compiles.

extern crate gilded_rose;

use gilded_rose::goblin::Item;

fn main() {
    let mut items = vec![
        Item {
            name: ProductTypes::DexterityVest.name(),
            sell_in: 10,
            quality: 20,
        },
        Item {
            name: ProductTypes::AgedBrie.name(),
            sell_in: 2,
            quality: 0,
        },
        Item {
            name: ProductTypes::ElixirOfTheMongoose.name(),
            sell_in: 5,
            quality: 7,
        },
        Item {
            name: ProductTypes::SulfurasHandOfRagnaros.name(),
            sell_in: 0,
            quality: 80,
        },
        Item {
            name: ProductTypes::BackstagePasses.name(),
            sell_in: 15,
            quality: 20,
        },
        Item {
            name: ProductTypes::ConjuredManaCake.name(),
            sell_in: 3,
            quality: 6,
        },
    ];

    for i in 0..50 {
        println!("Day {}:\n========================================", i);
        for item in &items {
            println!("{:?}", item);
        }
        update_quality(&mut items[..]);
    }
}

enum ProductTypes {
    AgedBrie,
    BackstagePasses,
    ConjuredManaCake,
    DexterityVest,
    ElixirOfTheMongoose,
    SulfurasHandOfRagnaros,
}

impl ProductTypes {
    fn name(&self) -> &str {
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

fn update_quality(items: &mut [Item]) {
    for i in 0..items.len() {
        if items[i].name != "Aged Brie"
            && items[i].name != "Backstage passes to a TAFKAL80ETC concert"
        {
            if items[i].quality > 0 {
                if items[i].name != "Sulfuras, Hand of Ragnaros" {
                    items[i].quality = items[i].quality - 1;
                }
            }
        } else {
            if items[i].quality < 50 {
                items[i].quality = items[i].quality + 1;

                if items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
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

        if items[i].name != "Sulfuras, Hand of Ragnaros" {
            items[i].sell_in = items[i].sell_in - 1;
        }

        if items[i].sell_in < 0 {
            if items[i].name != "Aged Brie" {
                if items[i].name != "Backstage passes to a TAFKAL80ETC concert" {
                    if items[i].quality > 0 {
                        if items[i].name != "Sulfuras, Hand of Ragnaros" {
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
        name: "+5 Dexterity Vest",
        sell_in: 10,
        quality: 20,
    }];
    update_quality(&mut items[..]);
    assert_eq!(items[0].sell_in, 9);
    assert_eq!(items[0].quality, 19);
}
