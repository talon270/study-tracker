// STUDY TRACKER · DESKTOP ENTRY
//   · Thin binary entry point only. All app wiring — window, tray,
//     notifications, updater — lives in lib.rs, whose `run()` doubles as the
//     Tauri mobile entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    study_tracker_lib::run();
}
