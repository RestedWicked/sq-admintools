use askama::Template;
use axum::Form;
use linked_hash_map::LinkedHashMap;
use serde::{ Deserialize, Serialize };
use tracing::info;

pub async fn sq_index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn sq_lootbox() -> LootboxTemplate {
    LootboxTemplate {}
}

#[derive(Template)]
#[template(path = "lootbox.html")]
pub struct LootboxTemplate;


#[derive(Deserialize)]
pub struct LootboxBluePrint {
    lootbox_name: String,
    panel_name: String,
}

enum LootboxType {
    Standard,
    Teaser,
}

struct PanelBoilerPlate {
    panel_name: String,
    lootbox_type: LootboxType,
    chance_command: String,
    perm: String,
    rows: String,
    title: String,
    commands_on_open: Vec<String>,
    panel_type: Vec<String>,
    item_details: ItemDetails,
}

struct ItemDetails {
    lootbox_slot: u8,
    material: String,
    name: String,
    commands: Vec<String>,
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

    #[serde(rename(serialize = "panelType"))]
    panel_type: Vec<String>,

    item: LinkedHashMap<u8, ItemSettings>
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


pub async fn gen_lootbox(Form(lootbox): Form<LootboxBluePrint>) {
    let item_details = ItemDetails {
        lootbox_slot: 4,
        material: "STONE".to_string(),
        name: "&fClick to Open!".to_string(),
        commands: Vec::new(),
    };

    let mut panel_bplate = PanelBoilerPlate {
        panel_name: lootbox.panel_name,
        lootbox_type: LootboxType::Teaser,
        chance_command: "set-data= chance %cp-random-1,33%".to_string(),

        perm: "default".to_string(),
        rows: "DROPPER".to_string(),
        title: lootbox.lootbox_name,
        commands_on_open: Vec::new(),
        panel_type: vec!["unclosable".to_string(), "nocommand".to_string()],
        item_details,
    };

    match panel_bplate.lootbox_type {
        LootboxType::Standard => {
            panel_bplate.commands_on_open = vec![panel_bplate.chance_command.clone()];
        }
        LootboxType::Teaser => {
            panel_bplate.item_details.commands = vec![panel_bplate.chance_command.clone()];
        }
    }
    
    let panels = main_panel(panel_bplate);

    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("lootbox.yml")
        .expect("Couldn't open file");

    serde_yaml::to_writer(f, &panels).unwrap();
    info!("lootbox.yml created!")
}

fn main_panel(boiler_plate: PanelBoilerPlate) -> Panels {
    Panels {
        panels: main_panel_map(boiler_plate),
    }
}

fn main_panel_map(boiler_plate: PanelBoilerPlate) -> LinkedHashMap<String, PanelConfig> {
    let mut main_panel = LinkedHashMap::new();
    main_panel.insert(boiler_plate.panel_name.clone(), panel_config(boiler_plate));
    main_panel
}

fn panel_config(boiler_plate: PanelBoilerPlate) -> PanelConfig {
    PanelConfig {
        perm: boiler_plate.perm,
        rows: boiler_plate.rows,
        title: boiler_plate.title,
        commands_on_open: boiler_plate.commands_on_open,
        panel_type: boiler_plate.panel_type,
        item: item_settings_map(boiler_plate.item_details),
    }
}

fn item_settings_map(item_details: ItemDetails) -> LinkedHashMap<u8, ItemSettings> {
    let mut item_settings_map = LinkedHashMap::new();
    item_settings_map.insert(
        item_details.lootbox_slot.clone(),
        item_settings(item_details),
    );
    item_settings_map
}

fn item_settings(item_details: ItemDetails) -> ItemSettings {
    ItemSettings {
        material: item_details.material,
        name: item_details.name,
        commands: item_details.commands,
        has: has_section_map(),
    }
}

fn has_section_map() -> LinkedHashMap<String, HasSections> {
    let mut has_section_map = LinkedHashMap::new();
    has_section_map.insert(String::from("has0"), has_section());
    has_section_map
}

fn has_section() -> HasSections {
    HasSections {
        value0: 1,
        compare0: String::from("%cp-data-chance%"),
        material: String::from("Stone"),
        stack: 1,
        name: String::from("&fClick to get!"),
        commands: vec![
            String::from("set-data chance 0"),
            String::from("msg- &fYou got $100"),
            String::from("console= cmi money give %cp-player-name% 100"),
            String::from("cpc"),
        ],
    }
}
