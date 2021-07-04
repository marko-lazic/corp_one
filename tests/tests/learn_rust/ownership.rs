#[test]
fn reference_test() {
    let mut vectr = vec![1, 2];
    let v = &mut vectr;
    v.push(3);

    println!("{:?}", vectr);
}

#[test]
fn memory_and_allocation() {
    let hello = "hello";

    let s1 = String::from("world");
    // Rust defaults to moving a value
    let _s2 = s1.clone(); // To copy string call clone()

    println!("{} {}", hello, s1);

    let x = 5;
    let _y = x; // Primitives have Copy trait by default for integer, booleans and character
}

#[test]
fn ownership_and_functions() {
    let s = String::from("hello");
    takes_ownership(s);
    // s is not moved to takes_ownership and destroyed there

    let x = 5;
    makes_copy(x); // x is copied and can be used again
    println!("{}", x);

    let s1 = gives_ownership();
    println!("s1 {}", s1);

    let s2 = String::from("hello"); // value is borrowed to takes_and_gives_back(String)
    let s3 = takes_and_gives_back(s2);
    println!("s3 {}", s3);
}

// Rust defaults to moving a value
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// Rust copies primitive
fn makes_copy(some_primitive: i32) {
    println!("{}", some_primitive);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

struct WordList {
    s1: String,
}
impl Default for WordList {
    fn default() -> Self {
        WordList { s1: String::new() }
    }
}

impl WordList {
    fn change(&mut self, some_string: &String) {
        self.s1.push_str(some_string);
    }
}

#[test]
fn references_and_borrowing() {
    let mut wl = WordList::default();
    wl.change(&String::from("hello"));
    wl.change(&String::from("world"));
    eprintln!("{}", wl.s1);
}

#[test]
fn some_funny_dereference_test() {
    let ptr = Box::new(42);

    println!("{}", ptr);

    let a: Vec<_> = vec![2, 3, 5].into_iter().map(|x| x * 2).collect();

    let b: Vec<_> = vec![1, 2, 3].iter().map(|x| x * 2).collect();

    println!("{:?}", a);
    println!("{:?}", b);
}
