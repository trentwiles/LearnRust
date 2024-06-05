// Rust is statically typed (you need to tell rust the type of each variable)
// Goodbye to the easy days of python...

// Guide to common types
// u32: unassigned 32-bit integer
// usize: arch, depends of the archetype of your computer (32 bit vs. 64 bit)
// f32: floating point 32 bit, for decimals

fn main(){

    
    let guess: u32 = "395".parse().expect("Not a number!");
    println!("Value of guess: {guess}");

    let floatExample: f32 = 3.14;
    println!("Float example: {floatExample}");

    // will not work
    // let unfloated: u32 = floatExample;
    // println!("unfloated: {unfloated}");


    // boolean
    let boolean: bool = true;
    let boolean: bool = false;

    // characters
    let chr: char = 'd';

    // tuple
    let exampleTuple: (u32, char, u8) = (384, 'x', 9);
    let (x,y,z) = exampleTuple;
    println!("x = {x}, y = {y}, z = {z}"); // x = 384, y = x, z = 9
    // you can also access elements directly like so
    let firstVal = exampleTuple.0;
    println!("first value = {firstVal}");


    // arrays
    let arr = [1,3,5];
    let arr2: [u32; 5] = [1,2,3,4,5];
    let secondElement = arr2[1];
    println!("Second element: {secondElement}")
}