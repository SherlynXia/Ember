use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_screen_size(app: AppHandle) -> Result<(u32, u32), String> {
    let window = app.get_webview_window("pet")
        .ok_or("Pet window not found")?;
    
    let monitor = window.current_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    
    let size = monitor.size();
    
    // 返回物理像素尺寸（与窗口位置使用相同的坐标系）
    Ok((size.width, size.height))
}

#[command]
pub fn get_pet_window_position(app: AppHandle) -> Result<(i32, i32), String> {
    let window = app.get_webview_window("pet")
        .ok_or("Pet window not found")?;
    
    let position = window.outer_position()
        .map_err(|e| e.to_string())?;
    
    Ok((position.x, position.y))
}

#[command]
pub fn move_pet_window(app: AppHandle, x: i32, y: i32) -> Result<(), String> {
    let window = app.get_webview_window("pet")
        .ok_or("Pet window not found")?;
    
    window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(x, y)))
        .map_err(|e| e.to_string())
}