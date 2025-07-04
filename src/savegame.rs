use crate::entity::GameState;

const CURRENT_SAVE_FILE: &str = "savegame.json";

pub fn save_game_state(game: &GameState) -> Result<(), String> {
    println!("Saving game state to {}", CURRENT_SAVE_FILE);
    let json = serde_json::to_string(game).map_err(|e| e.to_string())?;
    std::fs::write(CURRENT_SAVE_FILE, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_game_state() -> Result<GameState, String> {
    if !savegame_exists() {
        return Err("No save file found".to_string());
    }
    println!("Loading save file from {}", CURRENT_SAVE_FILE);
    let json = std::fs::read_to_string(CURRENT_SAVE_FILE).map_err(|e| e.to_string())?;
    let game: GameState = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(game)
}

pub fn savegame_exists() -> bool {
    std::path::Path::new(CURRENT_SAVE_FILE).exists()
}

pub fn delete_save() -> Result<(), String> {
    if !savegame_exists() {
        return Ok(());
    }
    println!("Deleting save file at {}", CURRENT_SAVE_FILE);
    std::fs::remove_file(CURRENT_SAVE_FILE).map_err(|e| e.to_string())?;
    Ok(())
}