use super::models::{FoodEntry, MacroNutrients};
use super::repository::NutritionRepository;

#[derive(Debug)]
pub struct LogFoodCommand {
    pub name: String,
    pub protein_g: f32,
    pub carbs_g: f32,
    pub fat_g: f32,
}

pub fn handle_log_food(
    cmd: LogFoodCommand,
    repo: &impl NutritionRepository
) -> Result<FoodEntry, String> {
    if cmd.protein_g < 0.0 || cmd.carbs_g < 0.0 || cmd.fat_g < 0.0 {
        return Err("Macros cannot be negative".to_string());
    }
    // 4kcal per gram of protein/carbs, 9kcal per gram of fat

    let calories = (cmd.protein_g * 4.0 + cmd.carbs_g * 4.0 + cmd.fat_g * 9.0).round() as u32;
    
    let entry = FoodEntry {
        name: cmd.name,
        macros: MacroNutrients {
            protein_g: cmd.protein_g,
            carbs_g: cmd.carbs_g,
            fat_g: cmd.fat_g,
            calories,
        },
        timestamp: "Just now".to_string(),
    };

    repo.save_food(&entry)?;

    Ok(entry)

}
