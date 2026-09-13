// STUDY TRACKER · DESKTOP SHELL
//   · WINDOW         loads the existing app (frontendDist = dist, the copied index.html)
//   · TRAY           stays resident so the JS reminder/timer scheduler keeps ticking
//   · CLOSE          hides to tray instead of quitting
//   · NOTIFICATIONS  tauri-plugin-notification, driven from JS via window.__TAURI__
//   · UPDATE         run_update/relaunch, driven from the Settings "Check for
//                    updates" button — see run_update() below for what it runs
//
// No app logic lives here. Everything the tracker does stays in index.html.
// This is the native frame and the tray contract, nothing more.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::process::Command;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, WindowEvent,
};

/* The project root, baked in at compile time from where this crate lives.
   Rebuilding from a different checkout re-bakes the right value, so nothing
   here is hand-maintained the way install.sh's $HOME lookup is. */
fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri always has a parent directory")
        .to_path_buf()
}

fn run_step(program: &str, args: &[&str], cwd: &PathBuf) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("couldn't start {program}: {e}"))?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if output.status.success() {
        Ok(text)
    } else {
        Err(format!("{program} {args:?} failed:\n{text}"))
    }
}

/* Rebuild + reinstall from the current index.html, so the Settings button
   does by hand what SETUP-desktop.md otherwise tells you to run in a
   terminal: copy-assets.sh, cargo build --release, install.sh. Runs on
   Tauri's blocking-command thread pool, not the UI thread, so the tray and
   window stay responsive during the up-to-a-minute cargo build. */
#[tauri::command]
fn run_update() -> Result<String, String> {
    let root = project_root();
    let cargo = if Command::new("cargo").arg("--version").output().is_ok() {
        "cargo".to_string()
    } else {
        // ponytail: GUI launches often don't inherit a login shell's PATH, so
        // rustup's default install location is the one fallback worth trying.
        // Anything more exotic (asdf, a custom toolchain dir) still needs a
        // terminal build, same as today.
        format!("{}/.cargo/bin/cargo", std::env::var("HOME").unwrap_or_default())
    };

    let mut log = run_step("sh", &["src-tauri/copy-assets.sh"], &root)?;
    log.push_str(&run_step(
        &cargo,
        &["build", "--release", "--manifest-path", "src-tauri/Cargo.toml"],
        &root,
    )?);
    log.push_str(&run_step("sh", &["src-tauri/install.sh"], &root)?);
    Ok(log)
}

#[tauri::command]
fn relaunch(app: AppHandle) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    Command::new(exe).spawn().map_err(|e| e.to_string())?;
    app.exit(0);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![run_update, relaunch])
        .setup(|app| {
            let show = MenuItem::with_id(app, "show", "Open Study Tracker", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Study Tracker")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Close = minimize to tray. Quitting the reminder scheduler with the
            // window is the silent-app failure this shell exists to avoid — a
            // pomodoro end or study reminder fires only while the JS is running.
            // Real quit is the tray menu's "Quit".
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Study Tracker");
}
