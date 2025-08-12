#[allow(unused)]

/* Public API */
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

/* Implementor Button */
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // drawing
        println!("drawing button!");
    }
}

/* Implementor SelectBox */
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing Select Box!");
    }
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


fn test_oop_1() {
    let screen = Screen {
        components: vec![
            Box::new(
                SelectBox {
                    width: 10,
                    height: 10,
                    options: vec![
                        String::from("Sometimes"),
                        String::from("Poor"),
                        String::from("Standard"),
                    ],
            }),
            Box::new(
                Button {
                    width: 10,
                    height: 10,
                    label: String::from("X"),
                }
            ),
        ]
    };

    screen.run();
}

fn main() {
    println!("This is an OOP sample example");
    test_oop_1();
    println!("Finished testing...");
}

