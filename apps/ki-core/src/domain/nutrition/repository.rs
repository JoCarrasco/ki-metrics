use super::models::FoodEntry;

pub trait NutritionRepository {
    fn save_food(&self, entry: &FoodEntry) -> Result<(), String>;
}
