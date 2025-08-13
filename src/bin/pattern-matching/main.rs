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
}

fn main() {
    println!("This is pattern matching rust!");
}

