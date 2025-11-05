use rdev::{listen, Event, EventType, Key};

fn main() {
    println!("Starting rdev listener test. Press Escape to see if it crashes.");
    println!("Press any other key to see its event.");

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::Escape) => {
            println!("Escape key pressed in test!");
        }
        _ => {
            println!("Event: {:?}", event);
        }
    }
}
