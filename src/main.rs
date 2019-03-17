#[derive(Debug)]
struct Foo {
    x: u64,
    y: f32,
    z: bool,
    s: &'static str,
}

// Exemple 1: ownership
fn take_ownership_foo(foo: Foo) {
    // Use it here
    println!("owned foo: {:#?}", foo);
    // foo and its content are dropped here
}

// Demonstrate one of the most Rust important rule
// the owership
fn ex_1() {
    let foo = Foo {x: 16_u64, y: 5 as f32, z: true, s: "fsdfsg"};
    take_ownership_foo(foo);
    // foo is not owned by this scope anymore
    // you cannot use it here.
    //println!("{:#?}", foo);
}

// EXEMPLE 2: borrowing
fn take_borrowing_foo(foo: &Foo) {
    println!("borrowed foo: {:#?}", *foo);
}

fn ex_2() {
    let foo = Foo {x: 16_u64, y: 5 as f32, z: true, s: "fsdfsg"};
    take_borrowing_foo(&foo);
    // take_borrowing_foo borrows the foo instance
    // and will restores the ownership of it here
    println!("after borrowing foo: {:#?}", foo); // works
}

// EXEMPLE 3: lifetimes annotations
/*
struct Engine {}

enum Brand {
    Wolkswagen,
    BMW,
    Audi,
    Mercedes,
}

struct Car<'a> {
    brand: Brand,
    engine: &'a Engine,
}

impl<'a> Car<'a> {
    fn new() -> Car<'a> {
        let engine = Engine {};
        Car {
            brand: Brand::Wolkswagen,
            engine: &engine,
        }
    }
}

fn ex_3() {
    let car = Car::new();
}
*/

fn main() {
    //ex_1();
    ex_2();
    //ex_3();
}
