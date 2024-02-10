//use axum_extra::extract::Form;
use axum::body::Bytes;
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use serde_qs::Config;
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

#[derive(Deserialize, Serialize, Debug)]
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
    #[serde(default)]
    item_has: Vec<Loot>,
//    #[serde(flatten)]
//    extra: LinkedHashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
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
    PERM,
}
impl FromStr for LootType {
    type Err = ();

    fn from_str(input: &str) -> Result<LootType, Self::Err> {
        match input {
            "IGC" => Ok(LootType::IGC),
            "SQUID" => Ok(LootType::SQUID),
            "COMMAND" => Ok(LootType::COMMAND),
            "PERM" => Ok(LootType::PERM),
            &_ => Err(()),
        }
    }
}

#[derive(Deserialize)]
struct LootCommand {
    loot_type: LootType,
    loot_command: String,
}

#[derive(Serialize)]
struct Panels {
    panels: LinkedHashMap<String, PanelConfig>,
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
    item: LinkedHashMap<String, ItemSettings>,
}

#[derive(Serialize)]
struct ItemSettings {
    material: String,
    name: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    commands: Vec<String>,

    #[serde(flatten)]
    has: LinkedHashMap<String, HasSection>,
}

#[derive(Serialize)]
struct HasSection {
    value0: usize,
    compare0: String,
    material: String,
    stack: u32,
    name: String,
    commands: Vec<String>,
}

pub async fn gen_lootbox(body: Bytes) {
    let config = Config::new(5, false);
    println!("{:#?}", body);

    let panel_bplate: LootboxBluePrint = config.deserialize_bytes(&body).unwrap();
    println!("{:#?}", panel_bplate);

    let file_name = format!("{}.yml", panel_bplate.panel_name);
    
    let mut panels =  LinkedHashMap::new();
    let item = LinkedHashMap::new();
    let mut has = LinkedHashMap::new();

    for (i, loot) in panel_bplate.item_has.iter().enumerate() {
        let section = HasSection {
            value0: i + 1,
            compare0: "&cp-data-chance&".to_string(),
            material: "STONE".to_string(),
            stack: 1,
            name: loot.loot_name.clone(),
            commands: vec!["Placeholder".to_string()],
        };
        has.insert(format!("has{}",i), section);
    }
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


    std::fs::create_dir_all("output/lootbox").unwrap();
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("output/lootbox/{}", &file_name))
        .expect("Couldn't open file");

    serde_yaml::to_writer(f, &lootbox).unwrap();

    info!("{} was created!", file_name)
}
