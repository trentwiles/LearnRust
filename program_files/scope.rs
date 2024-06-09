fn main(){
    // s is not valid here
    {
        let s = "yass!!!"; // s is valid here
    }


    // Second type of string, "String"
    // From rust's docs:
    // "manages data allocated on the heap and as such is
    // able to store an amount of text that is unknown to us at compile time"
    // It isn't a literal
    // SIMILAR to StringBuilder in Java, and must be mutable
    let mut s = String::from("hello");
    // this string we just created is mutable
    // just like .append() in StringBuilder
    s.push_str(" everyone...");
    println!("{}", s);
    // hello everyone...


    // here's a test for the stack
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // here's another test for the stack... is there anything different?
    // let s3 = String::from("hello");
    // let s4 = s3;

    // println!("s3 = {}, s4 = {}", s3, s4);
    //
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // this won't work, we need to do a deep copy, because of stack issues
}