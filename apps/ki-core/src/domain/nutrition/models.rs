#[derive(Debug, Clone)]
pub struct MacroNutrients {
    pub protein_g: f32,
    pub carbs_g: f32,
    pub fat_g: f32,
    pub calories: u32,
}
#[derive(Debug, Clone)]
pub struct FoodEntry {
    pub name: String,
    pub macros: MacroNutrients,
    pub timestamp: String,
}
