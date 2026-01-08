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

fn generic<T>(n: T) -> i32
where T: MyTrait + Into<i32> {
    n.hello();
    100i32 + n.into()
}

fn main() {
    let n = 4;
    n.hello();

    let n = "world!";
    n.hello();

    println!("{}", generic(3));
}
