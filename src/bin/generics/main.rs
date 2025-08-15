
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn len(&self) {
        println!("This is the default case");
    }
}

impl<T> Point<T, T> {
    fn len(&self) {
        println!("Same type case");
    }
}

fn test_traits_2() {
    let p = Point {
        x: 32,
        y: 1.0,
    };

    let p2 = Point {
        x: 32,
        y: 32,
    };

    p.len();
    p2.len();
}

fn test_traits() {
    let v_int = vec![1, 2, 3, 1, 3, 44, 543 , 0];
    //println!("Largest: {}", largest(&v_int));
    let p_i = Point {
        x: 1,
        y: 2,
    };
}

fn main() {
    test_traits_2();
    println!("testing generics");
}

