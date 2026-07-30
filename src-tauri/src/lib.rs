#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use tauri::Manager;
use tauri::Url;

const POCKET_NODE_URL: &str = "http://127.0.0.1:8787";
const HEALTH_ADDR: &str = "127.0.0.1:8787";

struct PocketNodeChild(Mutex<Option<Child>>);

fn bundled_pocket_node_dir(app: &tauri::AppHandle) -> Option<PathBuf> {
    let resource = app.path().resource_dir().ok()?;
    let dir = resource.join("pocket-node");
    if dir.join("run-serve.sh").is_file() {
        return Some(dir);
    }
    None
}

fn pocket_node_listening() -> bool {
    let addr: SocketAddr = HEALTH_ADDR.parse().expect("health addr");
    TcpStream::connect_timeout(&addr, Duration::from_millis(400)).is_ok()
}

fn spawn_bundled_pocket_node(dir: &PathBuf) -> Option<Child> {
    let script = dir.join("run-serve.sh");
    Command::new("sh")
        .arg(script)
        .current_dir(dir)
        .spawn()
        .ok()
}

fn wait_for_pocket_node(max_wait: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < max_wait {
        if pocket_node_listening() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(400));
    }
    pocket_node_listening()
}

fn navigate_main_to_pocket_node(app: &tauri::AppHandle) {
    if let Ok(url) = Url::parse(POCKET_NODE_URL) {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.navigate(url);
        }
    }
}

fn stop_pocket_node_child(app: &tauri::AppHandle) {
    if let Some(state) = app.try_state::<PocketNodeChild>() {
        if let Ok(mut guard) = state.0.lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.kill();
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            if let Some(bundle_dir) = bundled_pocket_node_dir(&handle) {
                if !pocket_node_listening() {
                    if let Some(child) = spawn_bundled_pocket_node(&bundle_dir) {
                        app.manage(PocketNodeChild(Mutex::new(Some(child))));
                    }
                }
                wait_for_pocket_node(Duration::from_secs(45));
                #[cfg(not(debug_assertions))]
                navigate_main_to_pocket_node(&handle);
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building Pocket Agent desktop")
        .run(|app_handle, event| {
            if matches!(event, tauri::RunEvent::Exit) {
                stop_pocket_node_child(app_handle);
            }
        });
}
