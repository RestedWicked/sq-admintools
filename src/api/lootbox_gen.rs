use std::str::FromStr;
use axum_extra::extract::Form;
use linked_hash_map::LinkedHashMap;
use serde::{ Deserialize, Serialize };
use tracing::info;

fn default_perm() -> String {
    "default".to_string()
}
fn default_rows() -> String {
    "DROPPER".to_string()
}
fn default_panel_type() -> Vec<String> {
    vec!["unclosable".to_string(), "nocommand".to_string()]
}
fn default_item_slot() -> String {
    "4".to_string()
}
fn default_item_hover() -> String {
    "&fClick to Open!".to_string()
}

#[derive(Deserialize, Serialize)]
pub struct LootboxBluePrint {
    panel_name: String,
    panel_title: String,
    lootbox_type: LootboxType,
    #[serde(default)]
    commands_on_open: Vec<String>,
    material: String,
    #[serde(default = "default_item_hover")]
    item_hover: String,
    #[serde(default)]
    item_commands: Vec<String>,
    // Set in Config
    #[serde(default = "default_perm")]
    perm: String,
    #[serde(default = "default_rows")]
    rows: String,
    #[serde(default = "default_panel_type")]
    panel_type: Vec<String>,
    #[serde(default = "default_item_slot")]
    item_slot: String,
    #[serde(flatten)]
    extra: LinkedHashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
enum LootboxType {
    Standard,
    Teaser,
}

#[derive(Deserialize, Serialize, Debug)]
struct Loot {
    main_loot_type: LootType,
    loot_name: String,
//    #[serde(default)]
//    loot_commands: Vec<LootCommand>,
//    loot_material: String,
//    loot_repeats: u32,
//    loot_min: u32,
//    loot_max: u32,
//    loot_step: u32,
}

#[derive(Deserialize, Serialize, Debug)]
enum LootType {
    IGC,
    SQUID,
    COMMAND,
    PERM
}
impl FromStr for LootType {
    type Err = ();

    fn from_str(input: &str) -> Result<LootType, Self::Err> {
        match input {
            "IGC" => Ok(LootType::IGC),
            "SQUID" => Ok(LootType::SQUID),
            "COMMAND" => Ok(LootType::COMMAND),
            "PERM" => Ok(LootType::PERM),
            &_ => Err(())

        }
    }
}

#[derive(Deserialize)]
struct LootCommand {
    loot_type : LootType,
    loot_command: String,
}

#[derive(Serialize)]
struct Panels {
    panels: LinkedHashMap<String, PanelConfig>
}


#[derive(Serialize)]
struct PanelConfig {
    perm: String,
    rows: String,
    title: String,
    #[serde(
        rename(serialize = "commands-on-open"),
        skip_serializing_if = "Vec::is_empty"
    )]
    commands_on_open: Vec<String>,
    panel_type: Vec<String>,
    item: LinkedHashMap<String, ItemSettings>
}

#[derive(Serialize)]
struct ItemSettings {
    material: String,
    name: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    commands: Vec<String>,

    #[serde(flatten)]
    has: LinkedHashMap<String, HasSections>,
}

#[derive(Serialize)]
struct HasSections {
    value0: u32,
    compare0: String,
    material: String,
    stack: u32,
    name: String,
    commands: Vec<String>,
}


pub async fn gen_lootbox(Form(panel_bplate): Form<LootboxBluePrint>) {

    let file_name = format!("{}.yml", panel_bplate.panel_name);
    let mut index: usize = 0;
    let mut loot_builder:Vec<&str> = vec![];
    let mut loot: Vec<Loot> = vec![];
    for (key, value) in panel_bplate.extra.iter() {
        let has_loot: Vec<&str> = key.split(|c| c== '[' || c == ']').collect();
        //println!("{:#?}", has_loot);
        let iloot = has_loot[1].parse::<usize>().unwrap();
        if index + 1  == iloot {
            index += 1;
            loot.push(Loot {
                main_loot_type: LootType::from_str(loot_builder[0]).unwrap(),
                loot_name: loot_builder[1].to_string(),
            });
            loot_builder = vec![];
        }
        if index == iloot {
            loot_builder.push(value);
            //println!("{:#?}", loot_builder);
        }
        //loot[has_loot[1].parse::<usize>().unwrap()].;
        //println!("{}: {}", key, value);

    }
    loot.push(Loot {
        main_loot_type: LootType::from_str(loot_builder[0]).unwrap(),
        loot_name: loot_builder[1].to_string(),
    });

    println!("{:#?}", loot);
    /*
    let mut panels =  LinkedHashMap::new();
    let item = LinkedHashMap::new();
    let has = LinkedHashMap::new();

    has.insert()
    let mut item_settings = ItemSettings {
        material: panel_bplate.material,
        name: panel_bplate.item_hover,
        commands: panel_bplate.item_commands,
        has,
    };

    let mut panel_config = PanelConfig {
        perm: panel_bplate.perm,
        rows: panel_bplate.rows,
        title: panel_bplate.panel_title,
        commands_on_open: panel_bplate.commands_on_open,
        panel_type: panel_bplate.panel_type,
        item,
    };
    
    let chance = "set-data= chance %cp-random-1,133%".to_string();
    match panel_bplate.lootbox_type {
        LootboxType::Standard => {
            panel_config.commands_on_open.insert(0, chance)
        }
        LootboxType::Teaser => {
            item_settings.commands.insert(0, chance)
        }
    }
    panel_config.item.insert(panel_bplate.item_slot, item_settings);
    panels.insert(panel_bplate.panel_name, panel_config);
    
    let lootbox = Panels { panels };
    */



    std::fs::create_dir_all("output/lootbox").unwrap();
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("output/lootbox/{}", &file_name))
        .expect("Couldn't open file");

    serde_yaml::to_writer(f, &panel_bplate).unwrap();
    
    info!("{} was created!", file_name)
}

