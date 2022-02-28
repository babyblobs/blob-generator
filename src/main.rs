use rand::distributions::WeightedIndex;
use rand::distributions::Distribution;
use std::collections::HashMap;
use std::time;
use rand::SeedableRng;
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;

const BASETYPES: [&str; 5] = ["Basic", "Cat", "Bunny", "Fluffy", "Poop"];
const BASETYPES_WEIGHTS: [f32; 5] = [1.0, 0.75, 0.75, 0.75, 0.75];
const T1DARKBASES: [&str; 6] = ["Aegean Teal", "Charcoal Obsidian", "Cherry Wine", "Cinnamon Chocolate", "Eggplant Pine", "Purple Berry"];
const T1DARKBASES_WEIGHTS: [u8; 6] = [1, 1, 1, 1, 1, 1];
const T1BRIGHTBASES: [&str; 44] = ["Apricot Sage", "Arctic Berry", "Arctic Helio", "Bittersweet Iris", "Butter Tiger", "Carnation Lavender", "Cerulean Eggplant", "Coral Lime", "Cornflower Fuchsia", "Egyptian Maya", "Electric Amethyst", "Electric Azure", "Electric Sky", "Emerald Lime", "Fandango Lemonade", "Grape Bubblegum", "Imperial Peach", "Iris Orchid", "Jade Space", "Lapis Turquoise", "Lavender Gam", "Mauve Lapis", "Mellow Lemon", "Mint Bubblegum", "Mint Denim", "Mulberry Byzantine", "Ocean Sand", "Ocean Seafoam", "Olive Yale", "Orchid Periwinkle", "Pearl Cloud", "Pineapple Taffy", "Pistachio Lollipop", "Rose Mint", "Rufous Tart", "Salmon Taffy", "Shadow Whisper", "Taffy Cerise", "Teal Coral", "Terracotta Sand", "Turquoise Mint", "Viola Bisque", "Viola Maya", "Wisteria Periwinkle"]; 
const T1BRIGHTBASES_WEIGHTS: [u8; 44] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
const T2DARKBASES: [&str; 3] = ["Liquid Deep Ocean", "Liquid Heather Eggplant", "Liquid Inky Cherry"];
const T2DARKBASES_WEIGHTS: [u8; 3] = [1, 1, 1];
const T2BRIGHTBASES: [&str; 107] = ["Bronze", "Camouflage", "Creamy Ocean Starfish", "Gold", "Liquid Aegean Coral", "Liquid Aegean Kelly", "Liquid Aegean Vanilla", "Liquid African Plum", "Liquid Amaranth Banana", "Liquid Apple Butter", "Liquid Apricot Eggplant", "Liquid Arctic Cobalt", "Liquid Arctic Periwinkle", "Liquid Azure Mint", "Liquid Baby Blue Banana Rose", "Liquid Banana Iris", "Liquid Banana Ocean", "Liquid Banana Peach", "Liquid Biscotti Hunter", "Liquid Botswana Agate", "Liquid Boysenberry Ocean", "Liquid Bumblebee Cantaloupe", "Liquid Buttermilk Tiffany", "Liquid Byzantine Apricot", "Liquid Caramel Cream", "Liquid Cerise Lemon Sky", "Liquid Cerulean Berry Lime", "Liquid Cherry Wine Cream", "Liquid Chiffon Sky", "Liquid Cinnamon Silk", "Liquid Cornflower Opal", "Liquid Cornflower Salmon", "Liquid Cornflower Wheat", "Liquid Cotton Parakeet", "Liquid Cream Crepe", "Liquid Cream Fern", "Liquid Creamy Cerulean Iris", "Liquid Cyan Sapphire", "Liquid Daffodil Orchid", "Liquid Denim Iris Ocean", "Liquid Denim Sepia", "Liquid Disco Confetti", "Liquid Electric Fuchsia", "Liquid Flamingo Punch", "Liquid Flamingo Sky Ocean", "Liquid Fuchsia Sky", "Liquid Holographic Iris", "Liquid Honey Rouge", "Liquid Independence Puce", "Liquid Indigo Mint Taffy", "Liquid Jam Candy", "Liquid Lapis Cobalt Geode", "Liquid Lapis Kelly", "Liquid Lavender Lime", "Liquid Lavender Sky", "Liquid Lavender Snow", "Liquid Lemon Cornflower", "Liquid Lemon Lime", "Liquid Lilac Butter Sky", "Liquid Lilac Macaroon", "Liquid Magenta Sky", "Liquid Mauve Teal Ruby", "Liquid Midnight Vanilla", "Liquid Minty Iris", "Liquid Mulberry Salt", "Liquid Mulberry Tea", "Liquid Mulberry Wheat", "Liquid Nacreous Pastel", "Liquid Navy Thistle", "Liquid Ocean Rose", "Liquid Ocean Tiger Geode", "Liquid Opaline Turquoise", "Liquid Parmesan Cantaloupe", "Liquid Pearl Thunder", "Liquid Peppermint Powder", "Liquid Periwinkle Carousel", "Liquid Periwinkle Maya Pearls", "Liquid Periwinkle Shadow", "Liquid Pine Mint Tea", "Liquid Pine Tea", "Liquid Pistachio Mauve", "Liquid Radiant  Cyandra", "Liquid Raisin Pine", "Liquid Raspberry Mint", "Liquid Redwood Oat", "Liquid Rose Mint", "Liquid Ruby Taupe", "Liquid Sapphire Yale", "Liquid Seafoam Iron", "Liquid Shortbread Moss", "Liquid Space Mint", "Liquid Spruce Tea", "Liquid Stone Blush Geode", "Liquid Strawberry Bumblebee", "Liquid Strawberry Eggplant", "Liquid Strawberry Laurel", "Liquid Tangerine Forest", "Liquid Tangerine Lollipop", "Liquid Thistle Banana Crepe", "Liquid Tortilla Slate Geode", "Liquid Turquoise Heather", "Liquid Ultra Cyan", "Liquid Viridian Tea", "Liquid White Hydrangea", "Platinum", "Silver", "Transparent"];
const T2BRIGHTBASES_WEIGHTS: [u8; 107] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
const T3DARKBASES: [&str; 9] = ["Black Diamond", "Cherry Soda", "Chocolate Drizzle", "Chocolate Sprinkles", "Closed Brain Coral", "Cola Soda", "Deep Space", "Grape Soda", "Purple Eclipse"];
const T3DARKBASES_WEIGHTS: [u8; 9] = [1, 1, 1, 1, 1, 1, 1, 1, 1];
const T3BRIGHTBASES: [&str; 53] = ["Amethyst", "Apple", "Blue Sapphire", "Blue Topaz", "Boba Tea", "Cherry", "Citrine", "Diamond", "Ebi", "Emerald", "Extraterrestrial Cosmos", "Gold Solar Flare", "Grooved Brain Coral", "Hamachi", "Hirame", "Hokkigai", "Ika", "Kani", "Kiwi Soda", "Margherita Pizza", "Milky Way Sky", "Olive Mushroom Pizza", "Onigiri", "Orange Foliaceous Coral", "Orange Soda", "Pepper Mushroom Pizza", "Pepperoni Onion Pizza", "Pepperoni Pizza", "Peridot", "Pineapple", "Pink Bubble Coral", "Pink Supernova", "Pink Tourmaline", "Purple Tube Sponge", "Raspberry Soda", "Red Nebula", "Ruby", "Saba", "Silver Comet", "Soda Water", "Strawberry Drizzle", "Strawberry Sprinkles", "Strawberry", "Tai", "Tako", "Tamago", "Turquoise Meteor", "Unagi", "Vanilla Drizzle", "Vanilla Sprinkles", "Veggie Pizza", "Watermelon", "Yellow Topaz"];
const T3BRIGHTBASES_WEIGHTS: [u8; 53] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

const FACES: [&str; 54] = ["Amazed", "Angry", "Astonished", "Blep", "Bloop", "Blush Laugh", "Blush Nya", "Blush", "Cry", "Dead Inside", "Dead", "Derp Blep", "Derp", "Disappointed", "Dizzy", "Drool", "Evil", "Flower Eyes", "Flustered", "Glower", "Hypnotized Drool", "Hypnotized", "Laugh", "Loveful", "Lovestruck", "Mad", "Melt", "Neutral Smile", "Plotting", "Sad", "Satisfied Nya", "Satisfied", "Shocked", "Side Eye", "Smile", "Smiling", "Smirk", "Smug", "Sparkle Eyes", "Spooked", "Squee", "Stargaze", "Starstruck Blep", "Starstruck", "Teary", "Tease", "Twinkle", "UwU", "Wail", "Weary", "Wink Laugh", "Wink", "Winkle", "Zany"];
const FACES_WEIGHTS: [f32; 54] = [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0, 0.2, 0.2, 0.5, 0.5, 0.5, 0.5, 1.0, 0.2, 0.2, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.5, 0.2, 0.2, 1.0, 0.2, 0.2, 0.2, 0.5, 1.0, 0.2, 1.0, 0.5, 1.0, 1.0, 1.0, 0.2, 0.5];

const T1BG: [&str; 10] = ["Arctic", "Cornflower", "Daffodil", "Indigo", "Lime", "Mint", "Peach", "Rose", "Steel", "Taffy"];
const T1BG_WEIGHTS: [u8; 10] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
const T2BG: [&str; 90] = ["Arctic1", "Arctic2", "Arctic3", "Arctic4", "Arctic5", "Arctic6", "Arctic7", "Arctic8", "Arctic9", "Cornflower1", "Cornflower2", "Cornflower3", "Cornflower4", "Cornflower5", "Cornflower6", "Cornflower7", "Cornflower8", "Cornflower9", "Daffodil1", "Daffodil2", "Daffodil3", "Daffodil4", "Daffodil5", "Daffodil6", "Daffodil7", "Daffodil8", "Daffodil9", "Indigo1", "Indigo2", "Indigo3", "Indigo4", "Indigo5", "Indigo6", "Indigo7", "Indigo8", "Indigo9", "Lime1", "Lime2", "Lime3", "Lime4", "Lime5", "Lime6", "Lime7", "Lime8", "Lime9", "Mint1", "Mint2", "Mint3", "Mint4", "Mint5", "Mint6", "Mint7", "Mint8", "Mint9", "Peach1", "Peach2", "Peach3", "Peach4", "Peach5", "Peach6", "Peach7", "Peach8", "Peach9", "Rose1", "Rose2", "Rose3", "Rose4", "Rose5", "Rose6", "Rose7", "Rose8", "Rose9", "Steel1", "Steel2", "Steel3", "Steel4", "Steel5", "Steel6", "Steel7", "Steel8", "Steel9", "Taffy1", "Taffy2", "Taffy3", "Taffy4", "Taffy5", "Taffy6", "Taffy7", "Taffy8", "Taffy9"];
const T2BG_WEIGHTS: [f32; 90] = [0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8, 0.5, 1.0, 1.0, 1.0, 0.5, 0.8, 0.8, 0.8, 0.8];

const PIZZA: [&str; 6] = ["Margherita Pizza", "Olive Mushroom Pizza", "Pepper Mushroom Pizza", "Pepperoni Onion Pizza", "Pepperoni Pizza", "Veggie Pizza"];
const SUSHI: [&str; 12] = ["Ebi", "Hamachi", "Hirame", "Hokkigai", "Ika", "Kani", "Onigiri", "Tai", "Tako", "Tamago", "Unagi", "Saba"];
const DRINK: [&str; 8] = ["Cherry Soda", "Cola Soda", "Grape Soda", "Boba Tea", "Kiwi Soda", "Orange Soda", "Raspberry Soda",  "Soda Water"];
const GEMSTONE: [&str; 11] = ["Black Diamond", "Amethyst", "Blue Sapphire", "Blue Topaz", "Citrine", "Diamond", "Emerald", "Yellow Topaz", "Ruby", "Peridot", "Pink Tourmaline"];
const DESSERT: [&str; 6] = ["Chocolate Drizzle", "Chocolate Sprinkles", "Strawberry Drizzle", "Strawberry Sprinkles",  "Vanilla Drizzle", "Vanilla Sprinkles"];
const FRUIT: [&str; 5] = ["Apple", "Cherry", "Pineapple", "Watermelon", "Strawberry"];
const CORAL: [&str; 5] = ["Closed Brain Coral", "Grooved Brain Coral", "Pink Bubble Coral", "Purple Tube Sponge", "Orange Foliaceous Coral"];
const GALAXY: [&str; 9] = ["Deep Space", "Purple Eclipse", "Extraterrestrial Cosmos", "Gold Solar Flare", "Milky Way Sky", "Red Nebula", "Silver Comet", "Turquoise Meteor", "Pink Supernova"];

enum Tier {
    T1,
    T2,
    T3,
}
use Tier::*;

fn create_combo<'a, R: rand::Rng>(mut rng: &mut R) -> HashMap<&'a str, &'a str> {

    let base_weights = WeightedIndex::new(BASETYPES_WEIGHTS).unwrap();
    let face_weights = WeightedIndex::new(FACES_WEIGHTS).unwrap();

    let mut traitmap = HashMap::new();

    let tier = {
        let tier_weights = WeightedIndex::new([0.8, 1.0, 0.4]).unwrap();
        &[T1, T2, T3][tier_weights.sample(&mut rng)]
    };

    match &tier {
        T1 => {traitmap.insert("Tier", "T1");},
        T2 => {traitmap.insert("Tier", "T2");},
        T3 => {traitmap.insert("Tier", "T3");},
    }

    let darkorbright = {
        match &tier {
            T1 => {
                let darkorbright_weights = WeightedIndex::new([T1DARKBASES.len(), T1BRIGHTBASES.len()]).unwrap();
                ["Dark", "Bright"][darkorbright_weights.sample(&mut rng)]
            },
            T2 => {
                let darkorbright_weights = WeightedIndex::new([T2DARKBASES.len(), T2BRIGHTBASES.len()]).unwrap();
                ["Dark", "Bright"][darkorbright_weights.sample(&mut rng)]
            },
            T3 => {
                let darkorbright_weights = WeightedIndex::new([T3DARKBASES.len(), T3BRIGHTBASES.len()]).unwrap();
                ["Dark", "Bright"][darkorbright_weights.sample(&mut rng)]
            },
        }
    };

    traitmap.insert("Shape", BASETYPES[base_weights.sample(&mut rng)]);

    //random dark or bright
    if darkorbright == "Dark" {
        traitmap.insert("Brightness", "Dark");
        match &tier {
            T1 => {
                let t1darkbase_weights = WeightedIndex::new(T1DARKBASES_WEIGHTS).unwrap();
                traitmap.insert("Blob Style", T1DARKBASES[t1darkbase_weights.sample(&mut rng)]);
            },
            T2 => {
                let t2darkbase_weights = WeightedIndex::new(T2DARKBASES_WEIGHTS).unwrap();
                traitmap.insert("Blob Style", T2DARKBASES[t2darkbase_weights.sample(&mut rng)]);
            },
            T3 => {
                let t3darkbase_weights = WeightedIndex::new(T3DARKBASES_WEIGHTS).unwrap();
                traitmap.insert("Blob Style", T3DARKBASES[t3darkbase_weights.sample(&mut rng)]);
            },
        }
    } else if darkorbright == "Bright" {
        traitmap.insert("Brightness", "Bright");
        match &tier {
            T1 => {
                let t1brightbase_weights = WeightedIndex::new(T1BRIGHTBASES_WEIGHTS).unwrap();
                traitmap.insert("Blob Style", T1BRIGHTBASES[t1brightbase_weights.sample(&mut rng)]);
            },
            T2 => {
                let t2brightbase_weights = WeightedIndex::new(T2BRIGHTBASES_WEIGHTS).unwrap();
                traitmap.insert("Blob Style", T2BRIGHTBASES[t2brightbase_weights.sample(&mut rng)]);
            },
            T3 => {
                let t3brightbase_weights = WeightedIndex::new(T3BRIGHTBASES_WEIGHTS).unwrap();
                traitmap.insert("Blob Style", T3BRIGHTBASES[t3brightbase_weights.sample(&mut rng)]);
            },
        }
    } else {
        panic!("Invalid Brightness");
    }

    traitmap.insert("Face", FACES[face_weights.sample(&mut rng)]);

    match &tier {
        T1 => {
            let t1_weights = WeightedIndex::new(T1BG_WEIGHTS).unwrap();
            traitmap.insert("Background", T1BG[t1_weights.sample(&mut rng)]);
        },
        T2 => {
            let t2_weights = WeightedIndex::new(T2BG_WEIGHTS).unwrap();
            traitmap.insert("Background", T2BG[t2_weights.sample(&mut rng)]);
        },
        T3 => {
            match traitmap.get("Blob Style").unwrap() {
                x if PIZZA.contains(x) => traitmap.insert("Background", "Pizza"),
                x if SUSHI.contains(x) => traitmap.insert("Background", "Sushi"),
                x if DRINK.contains(x) => traitmap.insert("Background", "Drink"),
                x if GEMSTONE.contains(x) => traitmap.insert("Background", "Gemstone"),
                x if DESSERT.contains(x) => traitmap.insert("Background", "Dessert"),
                x if FRUIT.contains(x) => traitmap.insert("Background", "Fruit"),
                x if CORAL.contains(x) => traitmap.insert("Background", "Coral"),
                x if GALAXY.contains(x) => traitmap.insert("Background", "Galaxy"),
                _ => panic!("Invalid Blob Style for T3"),
            };
        },
    }

    if (std::path::Path::new(&format!("Z:\\NFT stuff\\Assets\\bases\\{}\\{}\\{}\\{}.png", traitmap.get("Shape").unwrap(), traitmap.get("Tier").unwrap(), traitmap.get("Brightness").unwrap(), traitmap.get("Blob Style").unwrap()))).exists() {
        traitmap
    } else {
        create_combo(rng)
    }

}

fn create_blob<'a, R: rand::Rng>(name: u16, rng: &mut R) -> HashMap<&'a str, &'a str> {
    let newcombo = create_combo(rng);
    //println!("{:?}", newcombo);

    let base_filepath = format!("Z:\\NFT stuff\\Assets\\bases\\{}\\{}\\{}\\{}.png", newcombo.get("Shape").unwrap(), newcombo.get("Tier").unwrap(), newcombo.get("Brightness").unwrap(), newcombo.get("Blob Style").unwrap());

    let face_filepath = match newcombo.get("Brightness").unwrap() {
        &"Bright" => format!("Z:\\NFT stuff\\Assets\\BLACKFACES\\{}.png", newcombo.get("Face").unwrap()),
        &"Dark" => format!("Z:\\NFT stuff\\Assets\\WHITEFACES\\{}.png", newcombo.get("Face").unwrap()),
        _ => panic!("Invalid Brightness")
    };
    
    let bg_filepath = match newcombo.get("Tier").unwrap() {
        &"T1" => format!("Z:\\NFT stuff\\Assets\\backgrounds\\T1\\{}.png", newcombo.get("Background").unwrap()),
        &"T2" => format!("Z:\\NFT stuff\\Assets\\backgrounds\\T2\\{}.png", newcombo.get("Background").unwrap()),
        &"T3" => format!("Z:\\NFT stuff\\Assets\\backgrounds\\T3\\bg\\{}.png", newcombo.get("Background").unwrap()),
        _ => panic!("Invalid Tier")
    };        

    //println!("{}", base_filepath);
    //println!("{}", face_filepath);
    //println!("{}", bg_filepath);

    let base_img = match image::open(&base_filepath) {
        Ok(img) => img,
        Err(e) => panic!("{}, {}", e, &base_filepath)
    };
    let face_img = match image::open(&face_filepath) {
        Ok(img) => img,
        Err(e) => panic!("{}, {}", e, &face_filepath)
    };
    let mut bg_img = match image::open(&bg_filepath) {
        Ok(img) => img,
        Err(e) => panic!("{}, {}", e, &bg_filepath)
    };
    image::imageops::overlay(&mut bg_img, &base_img, 0, 0);
    image::imageops::overlay(&mut bg_img, &face_img, 0, 0);

    if newcombo.get("Tier").unwrap() == &"T3" {
        // if std::path::Path::new(&format!("Z:\\NFT stuff\\Assets\\backgrounds\\T3\\fg\\{}.png", newcombo.get("Background").unwrap())).exists() {
        //     let fg_img = image::io::Reader::open(format!("Z:\\NFT stuff\\Assets\\backgrounds\\T3\\fg\\{}.png", newcombo.get("Background").unwrap())).unwrap().decode().unwrap();
        //     image::imageops::overlay(&mut bg_img, &fg_img, 0, 0);
        // }
        let fp = format!("Z:\\NFT stuff\\Assets\\backgrounds\\T3\\fg\\{}.png", newcombo.get("Background").unwrap());
        let filepath = std::path::Path::new(&fp);
        match image::io::Reader::open(&filepath) {
            Ok(img) => {
                let fg_img = img.decode().unwrap();
                image::imageops::overlay(&mut bg_img, &fg_img, 0, 0);
            },
            Err(_) => {},
        };
    };

    //save blob
    let blob_filepath = format!("Z:\\Programming stuff\\Rust files\\dnagenerator\\Output\\{}.png", name);
    bg_img.save(blob_filepath).unwrap();

    return newcombo;

}

fn main() {
    
    let total_blobs = 8078;

    let now = time::Instant::now();

    // for name in 0..total_blobs {
    //     traits.push(create_blob(name, &mut rng));
    // }
    
    //redo that in parallel using multi threading

    let traits = (0..total_blobs)
    .into_par_iter()
    .map(|x| create_blob(x, &mut rand_chacha::ChaCha12Rng::seed_from_u64(69420+x as u64)))
    .collect::<Vec<HashMap<&str, &str>>>();

    let elapsed_time = now.elapsed();

    println!("Image processing took: {} seconds", elapsed_time.as_secs());

    //save traits
    let traits_filepath = "Z:\\Programming stuff\\Rust files\\dnagenerator\\Output\\traits.json";
    let mut file = File::create(traits_filepath).unwrap();
    let traits_json = format!("{:#?}", traits);
    file.write_all(traits_json.as_bytes()).unwrap();
}
