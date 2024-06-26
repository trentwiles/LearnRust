fn main(){
    // ways to create a string
    let mut one = String::new();
    println!("{}", one);

    let two = "hey";
    let two_second = two.to_string();

    // another way to make a valid string, note that this will always be UTF-8 encoded
    // to use push_str, must be mutable
    let mut three = String::from("this is anything");
    three.push_str("hey");
    println!("{}", three);
    // "this is anythinghey"
    // can also do this

    println!("{}", (three + "j"));
    // "this is anythingj"

    // Another important note: rust strings do not support indexing
    // Which means that you cannot do something like this:
    // println!("{}", three[0]);

    // INSTEAD...
    let four = String::from("Welcome!");
    // MAKE SURE TO USE & to ensure they are only references!!!!
    println!("1) {}, 2) {}, 3) {}", &four[0..1], &four[1..2], &four[2..3]);


    // String iteration
    let mut count = 0;
    for c in "hey".chars(){
        print!("-{c}-{count}");
        count += 1;
    }
}