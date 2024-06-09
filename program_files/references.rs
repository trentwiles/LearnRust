fn main() {
    let mut s = String::from("hello");

    // For sending a mutable variable into the function, we need to tell the function it is mutable
    // We do this by 
    let result: String = change(&mut s);
    println!("got {result}")


    // Note about mutable borrowing
    // Cannot borrow more than one at a time
    // For example this will not work:
    // let r1 = &mut s;
    // let r2 = &mut s;

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}