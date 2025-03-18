use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use std::time::Duration;
use tauri::async_runtime::Mutex;
use tokio::time::sleep;

#[tauri::command]
async fn simulate_barcodes(
    barcodes: Vec<String>,
    initial_delay_ms: u64,
    barcode_delay_ms: u64,
) -> Result<(), String> {
    if barcodes.is_empty() {
        return Ok(());
    }

    // Create thread-safe Enigo instance
    let enigo = Mutex::new(
        Enigo::new(&Settings::default())
            .map_err(|e| format!("Input initialization failed: {}", e))?,
    );

    // Convert milliseconds to Duration
    let initial_delay = Duration::from_millis(initial_delay_ms);
    let barcode_delay = Duration::from_millis(barcode_delay_ms);

    // Initial delay
    sleep(initial_delay).await;

    // Process first barcode
    {
        let mut enigo_guard = enigo.lock().await;
        enigo_guard
            .text(&barcodes[0])
            .map_err(|e| format!("Text input failed: {}", e))?;
        enigo_guard
            .key(Key::Return, Click)
            .map_err(|e| format!("Enter key failed: {}", e))?;
    }

    // Process remaining barcodes
    for barcode in &barcodes[1..] {
        sleep(barcode_delay).await;
        {
            let mut enigo_guard = enigo.lock().await;
            enigo_guard
                .text(barcode)
                .map_err(|e| format!("Text input failed: {}", e))?;
            enigo_guard
                .key(Key::Return, Click)
                .map_err(|e| format!("Enter key failed: {}", e))?;
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![simulate_barcodes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
