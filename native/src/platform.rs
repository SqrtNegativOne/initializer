use std::path::PathBuf;

const PING_WAV: &[u8] = include_bytes!("../assets/ping.wav");

pub fn launch_target(target: &str) {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let result = std::process::Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(target)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();

    if let Err(error) = result {
        log_error(&format!("Failed to launch {target}: {error}"));
    }
}

pub fn play_ping() {
    use std::ffi::c_void;

    #[link(name = "winmm")]
    extern "system" {
        fn PlaySoundW(psz_sound: *const u16, hmod: *mut c_void, fdw_sound: u32) -> i32;
    }

    const SND_ASYNC: u32 = 0x0001;
    const SND_NODEFAULT: u32 = 0x0002;
    const SND_MEMORY: u32 = 0x0004;

    unsafe {
        PlaySoundW(
            PING_WAV.as_ptr() as *const u16,
            std::ptr::null_mut(),
            SND_ASYNC | SND_NODEFAULT | SND_MEMORY,
        );
    }
}

pub fn apply_backdrop(cc: &eframe::CreationContext<'_>) {
    use window_vibrancy::{apply_acrylic, apply_blur};

    if apply_acrylic(cc, Some((18, 18, 18, 90))).is_err() {
        let _ = apply_blur(cc, Some((18, 18, 18, 90)));
    }
}

pub fn log_error(message: &str) {
    use std::io::Write;

    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let directory = base.join("Genesis");
    let _ = std::fs::create_dir_all(&directory);

    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(directory.join("genesis.log"))
    {
        let _ = writeln!(file, "{message}");
    }
}
