use std::env::current_dir;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

use schemars::{schema::RootSchema, schema_for};

use nft::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use nft::types::{State, TokenId};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(TokenId), &out_dir);
}

/// Writes schema to file. Overwrites existing file.
/// Panics on any error writing out the schema.
fn export_schema(schema: &RootSchema, out_dir: &PathBuf) {
    let title = schema
        .schema
        .metadata
        .as_ref()
        .map(|b| b.title.clone().unwrap_or_else(|| "untitled".to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    let path = out_dir.join(format!("{}.json", to_snake_case(&title)));
    let json = serde_json::to_string_pretty(schema).unwrap();
    write(&path, json + "\n").unwrap();
    println!("Created {}", path.to_str().unwrap());
}

fn to_snake_case(name: &str) -> String {
    let mut out = String::new();
    for (index, ch) in name.char_indices() {
        if index != 0 && ch.is_uppercase() {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}
