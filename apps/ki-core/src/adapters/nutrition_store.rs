use rusqlite::params;
use super::sqlite_db::SqliteDb;
use crate::domain::nutrition::repository::NutritionRepository;
use crate::domain::nutrition::models::FoodEntry;

impl NutritionRepository for SqliteDb {
    fn save_food(&self, entry: &FoodEntry) -> Result<(), String> {
        self.conn.execute(
            "INSERT INTO food_logs (name, protein, carbs, fat, calories, logged_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
            params![
              entry.name,
              entry.macros.protein_g,
              entry.macros.carbs_g,
              entry.macros.fat_g,
              entry.macros.calories,
              entry.timestamp
            ],
        ).map_err(|e| e.to_string())?;

        Ok(())
    }
}

