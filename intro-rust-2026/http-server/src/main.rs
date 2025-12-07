fn main() {
    let n: i32 = 4; // typed, stack
    let vector = vec![1, 2, 3]; // auto-detect type, heap

    println!("Hello, world!");
    println!("n is {} - vector is {:?}", n, vector);
}
