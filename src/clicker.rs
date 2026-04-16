use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub fn run(active: Arc<AtomicBool>, interval_ms: Arc<AtomicU64>) {
    loop {
        if active.load(Ordering::Relaxed) {
            click();
            std::thread::sleep(Duration::from_millis(interval_ms.load(Ordering::Relaxed)));
        } else {
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}

fn click() {
    let source = match CGEventSource::new(CGEventSourceStateID::Private) {
        Ok(s) => s,
        Err(_) => return,
    };
    let pos = match CGEvent::new(source.clone()) {
        Ok(e) => e.location(),
        Err(_) => return,
    };

    if let Ok(e) = CGEvent::new_mouse_event(source.clone(), CGEventType::LeftMouseDown, pos, CGMouseButton::Left) {
        e.post(CGEventTapLocation::HID);
    }
    std::thread::sleep(Duration::from_micros(500));
    if let Ok(e) = CGEvent::new_mouse_event(source, CGEventType::LeftMouseUp, pos, CGMouseButton::Left) {
        e.post(CGEventTapLocation::HID);
    }
}
