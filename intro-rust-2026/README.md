# Rust programming language

## About me

* Daniel García Moreno <dani@danigm.net> (@danigm)
* Ex US / Ex SUGUS (2005-2008)
* GNOME developer (evince, fractal, gtranslator)
* SUSE packaging team, Python Engineer
* https://danigm.net

## About Rust

A language empowering everyone
to build reliable and efficient software.

 * https://rust-lang.org/

## Creating a rust project with cargo

 * cargo: Rust's package manager

```
$ cargo new http-server
$ cd http-server
$ cargo run
```

 * What is Cargo.toml
 * Main source file in src/main.rs

### Extra

 * add: Add external dependencies (https://crates.io)
 * test: Run test suite
 * doc: Build doc
 * publish: Send lib to crates.io

Look for more helpful command with:

```
cargo --list
```

## Ownership, Borrowing

 * Variable definiton with `let`:

```
let n: i32 = 4; // typed, stack
let vector = vec![1, 2, 3]; // auto-detect type, heap
```

 * Let's try to modify these variables

```
n = 5;
vector[0] = n;
```

 * Variables are inmutable by default, use the `mut` prefix to make a variable
   mutable.

### Ownership rules

 1. Each value in Rust has an owner.
 2. There can only be one owner at a time.
 3. When the owner goes out of scope, the value will be dropped.

 * Let's try something new, let's *move* the vector to a new variable:

```
let newv = vector;
println!("This is the new vector variable {:?}", newv);
println!("This is the old vector variable {:?}", vector);
```

### Borrowing (references)

Data can't have more than one owner, but we can always borrow with `&` and
`&mut`:

```
let newv: &Vec<i32> = &vector;
println!("This is the new vector variable {:?}", newv);
println!("This is the old vector variable {:?}", vector);
```

There can be as much references (`&`) to a value as you want, but you can just
have one mutable reference `&mut`, and you can't merge normal references and
mutable references. These restrictions prevent data races.

## Let's start to write our server

To have a basic HTTP server we need to open a socket and listen for
connections, so let's take a look to the rust reference:
https://doc.rust-lang.org/stable/std/net/index.html

Rust comes with a good standard library, so we looks like there's a **Struct**
for our specific user case: **TcpListener**
https://doc.rust-lang.org/stable/std/net/struct.TcpListener.html

```
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    println!("Client connected");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
```

 * **use** statement at the top, to *import* definitions from other *modules*
 * **fn** statement for function definition, we've seen the main definition before
 * **for** statement, loop for each item in a collection
 * **?** operand, propagate errors with Result type, if is\_err, return Err

## Read and write to the stream (intro to traits)

**Traits** is the way to define shared behavior in Rust. Similar to Interfaces
in OOP, but with some key differences:

 * Traits can be implemented in any type, including primitives
 * You can define default implementation in traits

```
trait MyTrait {
    fn hello(&self) {
        println!("Hello world");
    }
}

impl MyTrait for i32 {}
impl MyTrait for &str {
    fn hello(&self) {
        println!("Custom hello");
    }
}

fn main() {
    let n = 4;
    n.hello();

    let n = "world!";
    n.hello();
}
```

More about traits:
 * Inheritance.
 * You can implement a trait in the type definition (struct) or in the trait
   definition.

The **TcpStream** type implements the **Read** and **Write** traits, so we can
read and write from/to the stream using the methods implemented in that trait:

* https://doc.rust-lang.org/std/io/trait.Read.html
* https://doc.rust-lang.org/std/io/trait.Write.html

```
fn handle_client(stream: &mut TcpStream) {
    println!("Client connected");
    let mut buf = [0u8; 256];
    let mut r = stream.read(&mut buf).unwrap();
    while r >= 256 {
        print!("{}", String::from_utf8(buf[0..r].to_vec()).unwrap());
        r = stream.read(&mut buf).unwrap();
    }
    println!("{}", String::from_utf8(buf[0..r].to_vec()).unwrap());

    stream.write(String::from("HTTP/1.1 200 OK\r\n\r\nHello world!\r\n").as_bytes());
    stream.write(String::from("\r\n").as_bytes());
}
```

## Implementing our own type in rust (struct)

Let's reorganize our code to create a custom type and have something similar to
Object Oriented Programming. We can have a new file called `server.rs` and
there we can declare our **struct**, and implement the *methods* using the
**impl**.

```
struct HttpServer { ... }

impl HttpServer { ... }

impl Display for HttpServer { ... }
```

We can make it printable implementing the **std::fmt::Display** trait.

Or we can use the `#[derive(Display)]` attribute to do it automatically, if the type is
simple enought.
https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html

## Let's handle errors correctly (intro to Result & Option types)

Rust implements errors handling using the type system. There's no exceptions,
just some default types to wrap results and optional values.

The **Option** type is used to return something that can be NULL:
https://doc.rust-lang.org/stable/std/option/enum.Option.html

For example we can pass an optional argument to change the server listening
port:

```
    pub fn new(port: Option<i32>) -> HttpServer {
        let default_port = port.unwrap_or(8080);
```

The **Result** type is used to return something that can fail:
https://doc.rust-lang.org/stable/std/result/enum.Result.html

We can modify our methods to remove all the plain **unwrap** code and handle
errors correctly or just *raise* to the caller.

```
    pub fn run(&self) -> Result<(), std::io::Error> {
    ...
    }
    fn handle_client(&self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
    ...
    }
```

It's typical to use pattern-matching (`match` or `if let`) to handle this kind
of errors with **Option** and **Result**

## TODO
 * Tests
 * Useful type wrappers (Box, Rc, Arc, Mutex)
