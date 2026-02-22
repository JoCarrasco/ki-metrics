use ki_core::domain::nutrition::commands::{LogFoodCommand, handle_log_food};
use ki_core::adapters::sqlite_db::SqliteDb;
use std::env;

fn main() {
    let db = SqliteDb::new("ki_metrics.db").expect("Failed to initialize database");
    let args: Vec<String> = env::args().collect();

    if (args.len() < 5) {
        println!("Usage: moon run ki-core:run -- <food_name> <protein> <carbs> <fat>");
        println!("Example: moon run ki-core:run -- Chicken 25.0 0.0 3.5");
    }

    let command = LogFoodCommand {
        name: args[1].clone(),
        protein_g: args[2].parse().unwrap_or(0.0),
        carbs_g: args[3].parse().unwrap_or(0.0),
        fat_g: args[4].parse().unwrap_or(0.0),
    };

    match handle_log_food(command, &db) {
        Ok(entry) => {
            println!("Success fully logged food!");
            println!("{}: {} kcal", entry.name, entry.macros.calories);
            println!("Protein: {}g | Carbs: {}g | Fat {}g", entry.macros.protein_g, entry.macros.carbs_g, entry.macros.fat_g);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

