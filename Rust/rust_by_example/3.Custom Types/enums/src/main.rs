/**
 * enum
 */

fn main() {
    // enum
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\"", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x = {}, y = {}", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum VeryVerboseEnum0fThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    type Operations = VeryVerboseEnum0fThingsToDoWithNumbers;

    let x = Operations::Add;
    let y = Operations::Subtract;
    impl VeryVerboseEnum0fThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
    println!("{}, {}", x.run(2, 2), y.run(3, 1));

    // use
}
