#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Launcher,
    Checklist,
    QuickJob,
}

#[derive(Clone, Copy)]
pub enum RoutineKind {
    Checklist(&'static str),
    Quit,
    QuickJob,
}

pub struct Routine {
    pub name: &'static str,
    pub kind: RoutineKind,
}

pub const ROUTINES: [Routine; 3] = [
    Routine {
        name: "genesis",
        kind: RoutineKind::Checklist("calibration.yaml"),
    },
    Routine {
        name: "speculatio",
        kind: RoutineKind::Quit,
    },
    Routine {
        name: "expeditum",
        kind: RoutineKind::QuickJob,
    },
];

pub fn routine_title(filename: &str) -> String {
    let stem = filename.split('.').next().unwrap_or(filename);
    let mut chars = stem.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().collect::<String>() + chars.as_str()
}
