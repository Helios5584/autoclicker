use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use tray_icon::TrayIconBuilder;

mod clicker;

/// (label, interval in ms)
const SPEEDS: &[(&str, u64)] = &[
    ("1 CPS", 1000),
    ("5 CPS", 200),
    ("10 CPS", 100),
    ("20 CPS", 50),
    ("50 CPS", 20),
    ("100 CPS", 10),
];
const DEFAULT_SPEED: usize = 2; // 10 CPS

/// Generates a simple 22x22 click-target icon (circle + center dot).
fn create_icon() -> tray_icon::Icon {
    let size = 22u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 / 2.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt();
            let idx = ((y * size + x) * 4) as usize;

            let is_ring = dist <= 9.0 && dist >= 7.5;
            let is_dot = dist <= 2.5;
            if is_ring || is_dot {
                rgba[idx + 3] = 255; // alpha only — template image uses alpha channel
            }
        }
    }

    tray_icon::Icon::from_rgba(rgba, size, size).expect("Failed to create icon")
}

fn main() {
    // Accessory = menu-bar-only app, no dock icon
    let mut event_loop = EventLoop::new();
    event_loop.set_activation_policy(ActivationPolicy::Accessory);

    // Shared state between main thread and clicker thread
    let active = Arc::new(AtomicBool::new(false));
    let interval_ms = Arc::new(AtomicU64::new(SPEEDS[DEFAULT_SPEED].1));

    // --- Menu ---
    let toggle_item = MenuItem::new("Start Clicking", true, None);

    let speed_submenu = Submenu::new("Click Speed", true);
    let speed_items: Vec<CheckMenuItem> = SPEEDS
        .iter()
        .enumerate()
        .map(|(i, (label, _))| {
            let item = CheckMenuItem::new(*label, true, i == DEFAULT_SPEED, None);
            speed_submenu.append(&item).unwrap();
            item
        })
        .collect();

    let hotkey_info = MenuItem::new("Emergency Off: ⌘⇧Esc", false, None);
    let quit_item = MenuItem::new("Quit", true, None);

    let menu = Menu::new();
    menu.append(&toggle_item).unwrap();
    menu.append(&speed_submenu).unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&hotkey_info).unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&quit_item).unwrap();

    // --- Tray icon ---
    let _tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(create_icon())
        .with_tooltip("AutoClicker")
        .build()
        .expect("Failed to create tray icon");

    // --- Global hotkey: Cmd+Shift+Escape ---
    let hotkey_manager = GlobalHotKeyManager::new().expect("Failed to init hotkey manager");
    let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Escape);
    hotkey_manager
        .register(hotkey)
        .expect("Failed to register emergency hotkey");

    // --- Clicker thread ---
    let clicker_active = active.clone();
    let clicker_interval = interval_ms.clone();
    thread::spawn(move || clicker::run(clicker_active, clicker_interval));

    // Capture IDs for event matching
    let toggle_id = toggle_item.id().clone();
    let quit_id = quit_item.id().clone();
    let speed_data: Vec<_> = speed_items
        .iter()
        .enumerate()
        .map(|(i, item)| (item.id().clone(), SPEEDS[i].1))
        .collect();

    // --- Event loop ---
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(50));

        // Menu events
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == toggle_id {
                let now_active = !active.load(Ordering::SeqCst);
                active.store(now_active, Ordering::SeqCst);
                toggle_item.set_text(if now_active {
                    "Stop Clicking"
                } else {
                    "Start Clicking"
                });
            } else if event.id == quit_id {
                active.store(false, Ordering::SeqCst);
                *control_flow = ControlFlow::ExitWithCode(0);
            } else {
                for (id, ms) in &speed_data {
                    if event.id == *id {
                        interval_ms.store(*ms, Ordering::SeqCst);
                        for item in &speed_items {
                            item.set_checked(false);
                        }
                        speed_items
                            .iter()
                            .find(|item| item.id() == id)
                            .unwrap()
                            .set_checked(true);
                        break;
                    }
                }
            }
        }

        // Hotkey: emergency off only
        while let Ok(_event) = GlobalHotKeyEvent::receiver().try_recv() {
            active.store(false, Ordering::SeqCst);
            toggle_item.set_text("Start Clicking");
        }
    });
}
