use std::fs;
use tauri::command;

#[command]
fn get_child_paths(path: String) -> Result<Vec<(String, bool)>, String> {
    let mut entries = vec![];
    let dir = fs::read_dir(&path).map_err(|e| e.to_string())?;
    for entry in dir {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let is_dir = path.is_dir();
        entries.push((path.display().to_string(), is_dir));
    }

    // sort folders first
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(entries)
}

#[command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[command]
fn write_file(path: String, contents: String) -> Result<(), String> {
    fs::write(&path, contents).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_file,
            write_file,
            get_child_paths
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
