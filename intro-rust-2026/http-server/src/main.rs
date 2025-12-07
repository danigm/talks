fn main() {
    let mut n: i32 = 4; // typed, stack
    let mut vector = vec![1, 2, 3]; // auto-detect type, heap

    n = 5;
    vector[0] = n;

    println!("Hello, world!");
    println!("n is {} - vector is {:?}", n, vector);

    let newv: &Vec<i32> = &vector;
    println!("This is the new vector variable {:?}", newv);
    println!("This is the old vector variable {:?}", vector);

}
