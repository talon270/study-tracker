// STUDY TRACKER · DESKTOP SHELL
//   · WINDOW         loads the existing app (frontendDist = dist, the copied index.html)
//   · TRAY           stays resident so the JS reminder/timer scheduler keeps ticking
//   · CLOSE          hides to tray instead of quitting
//   · NOTIFICATIONS  tauri-plugin-notification, driven from JS via window.__TAURI__
//   · UPDATE         run_update/relaunch, driven from the Settings "Check for
//                    updates" button — async, off the UI thread; see
//                    run_update() below for what it runs
//   · ROOT           the source tree is found at runtime, not baked in — see
//                    project_root(); a moved project must not break the updater
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

/* Where install.sh records the source tree it installed from, so a moved
   project can be pointed at again without a rebuild. */
fn root_hint_file() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_default())
        .join(".local/share/study-tracker/source-root")
}

/* The project root, resolved at RUNTIME.

   It used to be `env!("CARGO_MANIFEST_DIR")` alone, on the reasoning that
   rebuilding from a different checkout re-bakes the right value. That is true
   and useless: the moment the folder moves, the baked path is gone, and the
   rebuild that would fix it is the very thing that stops working. Moving this
   project from ~/SyncedWork/cach/Claude/Study to ~/SyncedWork/Claude/Study
   turned the Update button into "couldn't start sh: No such file or directory"
   — because a missing `current_dir` makes Command::output() fail with ENOENT
   and name the program rather than the directory.

   So: the environment override first (an escape hatch that needs no rebuild),
   then whatever install.sh recorded, then the compile-time location. A
   candidate only counts if it actually holds src-tauri/Cargo.toml — an empty
   or half-moved directory is not the source tree. None means we genuinely
   cannot find it, which callers report as that rather than as a missing `sh`. */
fn project_root() -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(v) = std::env::var("STUDY_TRACKER_ROOT") {
        if !v.trim().is_empty() {
            candidates.push(PathBuf::from(v.trim()));
        }
    }
    if let Ok(v) = std::fs::read_to_string(root_hint_file()) {
        if !v.trim().is_empty() {
            candidates.push(PathBuf::from(v.trim()));
        }
    }
    if let Some(baked) = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent() {
        candidates.push(baked.to_path_buf());
    }

    pick_root(candidates)
}

/* The part that got this wrong, split out so it can be tested without an
   environment: first candidate that actually holds src-tauri/Cargo.toml wins.
   A path that no longer exists is skipped rather than handed to current_dir,
   which is the whole bug. */
fn pick_root(candidates: Vec<PathBuf>) -> Option<PathBuf> {
    candidates
        .into_iter()
        .find(|p| p.join("src-tauri").join("Cargo.toml").is_file())
}

/* One message for "the source tree has moved", written where the user will
   actually read it: the Settings card. */
fn missing_root_error() -> String {
    format!(
        "Can't find the project folder this app was built from. It was at {baked}, \
         which no longer exists — the folder has been moved or renamed.\n\n\
         Fix it either way:\n\
         · run src-tauri/install.sh from the new location (records the new path, no rebuild), or\n\
         · launch with STUDY_TRACKER_ROOT=/path/to/Study set.",
        baked = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "?".into()),
    )
}

fn run_step(program: &str, args: &[&str], cwd: &PathBuf) -> Result<String, String> {
    /* Checked before spawning, because Command::output() reports a missing cwd
       as ENOENT against the PROGRAM — which is how a moved project folder came
       out as "couldn't start sh: No such file or directory" and sent the blame
       to the wrong place entirely. */
    if !cwd.is_dir() {
        return Err(format!(
            "working directory {} doesn't exist, so {program} was never started.",
            cwd.display()
        ));
    }
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
   terminal: copy-assets.sh, cargo build --release, install.sh.

   `async fn` is deliberate, not decoration: a plain sync #[tauri::command]
   runs INLINE on the thread that handles the IPC call (tauri-macros'
   body_blocking calls the function directly, no thread pool involved), which
   is the window's own event-loop thread. A sync run_update() froze the whole
   window for the full cargo build — reported as the app "crashing" — because
   nothing pumped window events while cargo ran. spawn_blocking moves the
   actual blocking work off that thread; the same fix already shipped in
   Helth's version of this file. */
#[tauri::command]
async fn run_update() -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(run_update_blocking)
        .await
        .map_err(|e| format!("update task panicked: {e}"))?
}

fn run_update_blocking() -> Result<String, String> {
    let root = project_root().ok_or_else(missing_root_error)?;
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


#[cfg(test)]
mod tests {
    use super::*;

    /* The exact failure this replaced: a baked path that no longer exists was
       passed straight to Command::current_dir, which fails with ENOENT and
       blames the program ("couldn't start sh"). */
    #[test]
    fn skips_a_path_that_no_longer_exists() {
        let real = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let dead = PathBuf::from("/definitely/not/here/Study");
        assert!(!dead.exists(), "test assumes this path is absent");

        assert_eq!(pick_root(vec![dead.clone()]), None);
        assert_eq!(pick_root(vec![dead, real.clone()]), Some(real));
    }

    /* A directory that exists but isn't the project is not the project. */
    #[test]
    fn rejects_a_directory_without_the_crate() {
        assert_eq!(pick_root(vec![PathBuf::from("/tmp")]), None);
    }

    /* Earlier candidates win, so STUDY_TRACKER_ROOT overrides the recorded and
       baked paths rather than merely being consulted. */
    #[test]
    fn honours_candidate_order() {
        let real = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        assert_eq!(
            pick_root(vec![
                PathBuf::from("/nonexistent"),
                real.clone(),
                PathBuf::from("/tmp"),
            ]),
            Some(real)
        );
    }
}
