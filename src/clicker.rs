use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Background loop: clicks at configured interval when active, idles otherwise.
pub fn run(active: Arc<AtomicBool>, interval_ms: Arc<AtomicU64>) {
    loop {
        if active.load(Ordering::Relaxed) {
            click();
            let ms = interval_ms.load(Ordering::Relaxed);
            std::thread::sleep(Duration::from_millis(ms));
        } else {
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}

/// Posts a left-click (down + up) at the current cursor position via CoreGraphics.
/// Uses HIDSystemState source to avoid interfering with accessibility event taps.
fn click() {
    let source = match CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Get current cursor position from a dummy event
    let pos = match CGEvent::new(source.clone()) {
        Ok(e) => e.location(),
        Err(_) => return,
    };

    // Mouse down
    if let Ok(down) = CGEvent::new_mouse_event(
        source.clone(),
        CGEventType::LeftMouseDown,
        pos,
        CGMouseButton::Left,
    ) {
        down.post(CGEventTapLocation::HID);
    }

    // Mouse up
    if let Ok(up) = CGEvent::new_mouse_event(
        source,
        CGEventType::LeftMouseUp,
        pos,
        CGMouseButton::Left,
    ) {
        up.post(CGEventTapLocation::HID);
    }
}
