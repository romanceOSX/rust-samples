struct Foo {
    content: String,
}

fn places_to_use_patterns() {
    // match arms
    let opt = Some(32);
    let foo = Some(Foo{content: String::new()});

    // TODO: how does ownership applies in this scenarios
    let a = match foo {
        None => None,
        Some(i) => Some(i),
    };

    let (_, a, b) = (1, 2, 3);
    println!("{a}, {b}");

    let my_tup  = (1, 2, 3, 4);

    match my_tup {
        (a, _, x, y) => println!("first case"),
        x => println!("Default case"),
    }
    let a = if true {
        32
    } else { 11 } ;
    println!("{a}");

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }

    match msg {
        Message::Hello {
            id: id_variable,
        } if id_variable < 10 && id_variable > 0 => println!("Found an id in range!!!!!!!: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

fn main() {
    println!("This is pattern matching rust!");
    places_to_use_patterns();
}

