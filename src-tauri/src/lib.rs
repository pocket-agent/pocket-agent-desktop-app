#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use tauri::Manager;
use tauri::Url;

const POCKET_NODE_URL: &str = "http://127.0.0.1:8787";
const HEALTH_ADDR: &str = "127.0.0.1:8787";
const WELCOME_PATH: &str = "/welcome";

struct PocketNodeChild(Mutex<Option<Child>>);

fn resource_root(app: &tauri::AppHandle) -> Option<PathBuf> {
    app.path().resource_dir().ok()
}

fn first_existing_dir(root: &Path, rel_paths: &[&str]) -> Option<PathBuf> {
    for rel in rel_paths {
        let dir = root.join(rel);
        if dir.join("run-serve.sh").is_file() {
            return Some(dir);
        }
    }
    None
}

fn bundled_pocket_node_dir(app: &tauri::AppHandle) -> Option<PathBuf> {
    let root = resource_root(app)?;
    first_existing_dir(
        &root,
        &[
            "resources/pocket-node",
            "pocket-node",
            "resources/pocket-node/pocket-node",
        ],
    )
}

fn bundled_startup_file(app: &tauri::AppHandle, name: &str) -> Option<PathBuf> {
    let root = resource_root(app)?;
    for rel in [
        format!("resources/startup/{}", name),
        format!("startup/{}", name),
    ] {
        let path = root.join(rel);
        if path.is_file() {
            return Some(path);
        }
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

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn navigate_main(app: &tauri::AppHandle, url: Url) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.navigate(url);
        show_main_window(app);
    }
}

fn navigate_main_app_file(app: &tauri::AppHandle, rel: &str) {
    if let Some(path) = bundled_startup_file(app, rel) {
        if let Ok(url) = Url::from_file_path(path) {
            navigate_main(app, url);
        }
    }
}

fn navigate_main_http(app: &tauri::AppHandle, path: &str) {
    let base = POCKET_NODE_URL.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    if let Ok(url) = Url::parse(&format!("{}/{}", base, path)) {
        navigate_main(app, url);
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

            #[cfg(not(debug_assertions))]
            {
                if let Some(window) = handle.get_webview_window("main") {
                    let _ = window.hide();
                }
                navigate_main_app_file(&handle, "index.html");
            }

            if let Some(bundle_dir) = bundled_pocket_node_dir(&handle) {
                if !pocket_node_listening() {
                    if let Some(child) = spawn_bundled_pocket_node(&bundle_dir) {
                        app.manage(PocketNodeChild(Mutex::new(Some(child))));
                    }
                }
                let ready = wait_for_pocket_node(Duration::from_secs(90));
                #[cfg(not(debug_assertions))]
                {
                    if ready {
                        navigate_main_http(&handle, WELCOME_PATH);
                    } else {
                        navigate_main_app_file(&handle, "failed.html");
                    }
                }
            } else {
                #[cfg(not(debug_assertions))]
                navigate_main_app_file(&handle, "failed.html");
            }

            #[cfg(debug_assertions)]
            if let Some(window) = handle.get_webview_window("main") {
                let _ = window.show();
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
