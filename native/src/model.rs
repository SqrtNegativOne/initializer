use std::path::PathBuf;

use serde::Deserialize;

use crate::platform::log_error;

#[derive(Clone, Deserialize)]
pub struct Task {
    pub text: String,
    #[serde(rename = "type", default)]
    pub kind: String,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub duration: Option<f64>,
}

pub fn load_checklist(filename: &str) -> Vec<Task> {
    let Some(directory) = find_checklists_dir() else {
        log_error("Could not locate the checklists directory");
        return Vec::new();
    };

    let path = directory.join(filename);
    match std::fs::read_to_string(&path) {
        Ok(contents) => match serde_yaml::from_str(&contents) {
            Ok(tasks) => tasks,
            Err(error) => {
                log_error(&format!("Failed to parse {}: {error}", path.display()));
                Vec::new()
            }
        },
        Err(error) => {
            log_error(&format!("Failed to read {}: {error}", path.display()));
            Vec::new()
        }
    }
}

fn find_checklists_dir() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        let mut current = exe.parent();
        for _ in 0..6 {
            let directory = current?;
            let candidate = directory.join("checklists");
            if candidate.is_dir() {
                return Some(candidate);
            }
            current = directory.parent();
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        let mut current = Some(cwd.as_path());
        for _ in 0..6 {
            let directory = current?;
            let candidate = directory.join("checklists");
            if candidate.is_dir() {
                return Some(candidate);
            }
            current = directory.parent();
        }
    }

    None
}
