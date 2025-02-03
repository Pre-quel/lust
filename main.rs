use serde_json::Value;
use std::fs;

pub struct Lust {
    pub data: Value,
    pub path: String,
}

pub struct Table {
    name: String,
}

impl Table {
    pub fn insert(&self, db: &mut Lust, record: Value) -> Result<(), Box<dyn std::error::Error>> {
        if !db.data[self.name.as_str()].is_array() {
            return Err("Table is not an array".into());
        }

        // yippe mutable borrowing, otherwise known as a weak reference in non-schizo terms
        if let Some(table_array) = db.data[self.name.as_str()].as_array_mut() {
            table_array.push(record);
            db.save()?;
            Ok(())
        } else {
            Err("Failed to insert record".into())
        }
    }

    pub fn where_str<'a>(&self, db: &'a Lust, query: &str) -> Vec<&'a Value> {
        let parts: Vec<&str> = query.split("==").map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Vec::new();
        }

        let key = parts[0];
        let value = parts[1].trim_matches('\'');

        if let Some(table_array) = db.data[self.name.as_str()].as_array() {
            table_array
                .iter()
                .filter(|record| record[key] == value)
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Lust {
    pub fn new(path: &str) -> Self {
        let data = fs::read_to_string(path)
            .and_then(|content| serde_json::from_str(&content).map_err(|e| e.into()))
            .unwrap_or_else(|_| serde_json::json!({}));

        Lust {
            data,
            path: path.to_string(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.path, json_string)?;
        Ok(())
    }

    pub fn init_table(&mut self, name: &str) {
        if !self.data.get(name).is_some() {
            self.data[name] = serde_json::json!([]);
        }
    }

    pub fn table(&self, name: &str) -> Table {
        Table {
            name: name.to_string(),
        }
    }
}

fn main() {
    let mut db = Lust::new("db.json");
    db.init_table("xyu");

    let table = db.table("xyu");

    let results = table.where_str(&db, "drug_name == 'Methamphetamine'");
    println!("Query Results: {:?}", results);
}

