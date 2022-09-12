#[tauri::command]
pub fn my_custom_command() -> Result<String, String> {
    // If something fails
    // Err("This failed!".into())
    // If it worked
    Ok("This worked!".into())
}
