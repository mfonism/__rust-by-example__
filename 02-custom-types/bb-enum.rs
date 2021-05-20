enum WebEvent {
    // an `enum` may either be unit-like
    PageLoad,
    PageUnload,
    // tuple struct like
    KeyPress(char),
    Paste(String),
    // or c struct like
    Click { x: i64, y: i64 },
}

// a function which takes a `WebEvent` enum as an argument
// and produces nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // can destructure enum in a match, too
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let clicked = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(load);
    let last = inspect(unload);
    // let's check out the _return value_ of the inspect function
    println!("{:?}", last);
}