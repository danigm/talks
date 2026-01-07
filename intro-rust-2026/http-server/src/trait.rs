trait MyTrait: std::fmt::Display {
    fn hello(&self) {
        println!("Hello world {self}");
    }
}

impl MyTrait for i32 {}
impl MyTrait for &str {
    fn hello(&self) {
        println!("Custom hello {self}");
    }
}

fn main() {
    let n = 4;
    n.hello();

    let n = "world!";
    n.hello();
}
