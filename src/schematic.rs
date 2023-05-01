use nbt::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchemFormat {
    #[serde(serialize_with = "nbt::i8_array")]
    pub block_data: Vec<i8>,
    pub block_entities: Vec<SchemBlockEntity>,
    pub data_version: i32,
    pub height: i16,
    pub length: i16,
    pub metadata: Metadata,
    #[serde(serialize_with = "nbt::i32_array")]
    pub offset: Vec<i32>,
    pub palette: HashMap<String, i32>,
    pub palette_max: i32,
    pub version: i32,
    pub width: i16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchemBlockEntity {
    pub id: String,

    #[serde(serialize_with = "nbt::i32_array")]
    pub pos: Vec<i32>,

    #[serde(flatten)]
    pub props: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Metadata {
    #[serde(rename = "WEOffsetX")]
    pub offset_x: i32,
    #[serde(rename = "WEOffsetY")]
    pub offset_y: i32,
    #[serde(rename = "WEOffsetZ")]
    pub offset_z: i32,
}
