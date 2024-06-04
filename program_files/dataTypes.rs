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
}