use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use tray_icon::TrayIconBuilder;

const SPEEDS: &[(&str, u64)] = &[
    ("1 CPS", 1000),
    ("5 CPS", 200),
    ("10 CPS", 100),
    ("20 CPS", 50),
    ("50 CPS", 20),
    ("100 CPS", 10),
];
const DEFAULT_SPEED: usize = 2;

fn create_icon() -> tray_icon::Icon {
    let size = 22u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];
    let c = size as f32 / 2.0;
    for y in 0..size {
        for x in 0..size {
            let d = ((x as f32 - c).powi(2) + (y as f32 - c).powi(2)).sqrt();
            if (d <= 9.0 && d >= 7.5) || d <= 2.5 {
                rgba[((y * size + x) * 4 + 3) as usize] = 255;
            }
        }
    }
    tray_icon::Icon::from_rgba(rgba, size, size).expect("icon")
}

fn click() {
    let src = match CGEventSource::new(CGEventSourceStateID::Private) {
        Ok(s) => s,
        Err(_) => return,
    };
    let pos = match CGEvent::new(src.clone()) {
        Ok(e) => e.location(),
        Err(_) => return,
    };
    if let Ok(e) = CGEvent::new_mouse_event(src.clone(), CGEventType::LeftMouseDown, pos, CGMouseButton::Left) {
        e.post(CGEventTapLocation::HID);
    }
    thread::sleep(Duration::from_micros(500));
    if let Ok(e) = CGEvent::new_mouse_event(src, CGEventType::LeftMouseUp, pos, CGMouseButton::Left) {
        e.post(CGEventTapLocation::HID);
    }
}

fn clicker_loop(active: Arc<AtomicBool>, interval_ms: Arc<AtomicU64>) {
    loop {
        if active.load(Ordering::Relaxed) {
            click();
            thread::sleep(Duration::from_millis(interval_ms.load(Ordering::Relaxed)));
        } else {
            thread::sleep(Duration::from_millis(50));
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let active = Arc::new(AtomicBool::new(false));
    let interval_ms = Arc::new(AtomicU64::new(SPEEDS[DEFAULT_SPEED].1));

    // Menu
    let toggle_item = MenuItem::new("Start Clicking", true, None);
    let speed_submenu = Submenu::new("Click Speed", true);
    let speed_items: Vec<CheckMenuItem> = SPEEDS.iter().enumerate().map(|(i, (label, _))| {
        let item = CheckMenuItem::new(*label, true, i == DEFAULT_SPEED, None);
        speed_submenu.append(&item).unwrap();
        item
    }).collect();
    let quit_item = MenuItem::new("Quit", true, None);

    let menu = Menu::new();
    menu.append(&toggle_item).unwrap();
    menu.append(&speed_submenu).unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&MenuItem::new("Toggle: ⌘⇧Esc", false, None)).unwrap();
    menu.append(&PredefinedMenuItem::separator()).unwrap();
    menu.append(&quit_item).unwrap();

    // Tray
    let _tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(create_icon())
        .with_tooltip("AutoClicker")
        .build()
        .expect("tray icon");

    // Hotkey
    let hk = GlobalHotKeyManager::new().expect("hotkey manager");
    hk.register(HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Escape))
        .expect("register hotkey");

    // Clicker thread
    let (ca, ci) = (active.clone(), interval_ms.clone());
    thread::spawn(move || clicker_loop(ca, ci));

    let toggle_id = toggle_item.id().clone();
    let quit_id = quit_item.id().clone();
    let speed_data: Vec<_> = speed_items.iter().enumerate()
        .map(|(i, item)| (item.id().clone(), SPEEDS[i].1)).collect();

    let do_toggle = |active: &AtomicBool, toggle_item: &MenuItem| {
        let now = !active.load(Ordering::SeqCst);
        active.store(now, Ordering::SeqCst);
        toggle_item.set_text(if now { "Stop Clicking" } else { "Start Clicking" });
    };

    event_loop.run(move |_, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(50));

        while let Ok(ev) = MenuEvent::receiver().try_recv() {
            if ev.id == toggle_id {
                do_toggle(&active, &toggle_item);
            } else if ev.id == quit_id {
                active.store(false, Ordering::SeqCst);
                *control_flow = ControlFlow::ExitWithCode(0);
            } else {
                for (id, ms) in &speed_data {
                    if ev.id == *id {
                        interval_ms.store(*ms, Ordering::SeqCst);
                        for item in &speed_items { item.set_checked(false); }
                        speed_items.iter().find(|item| item.id() == id).unwrap().set_checked(true);
                        break;
                    }
                }
            }
        }

        while let Ok(ev) = GlobalHotKeyEvent::receiver().try_recv() {
            if ev.state == global_hotkey::HotKeyState::Pressed {
                do_toggle(&active, &toggle_item);
            }
        }
    });
}
