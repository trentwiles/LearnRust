fn main() {
    // You MUST declare x as mutable, if you would like to change the value of it
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // Constants: (equivalent to "const" and "final" in javascript and java, respectively)
    const HELLO_WORLD: &str = "Hello world!";

    // SHADOWING: overwriting a variable
    let test = 5;
    println!("The value of the test variable is {test}");

    let test = 50;
    println!("The value of the test variable is NOW {test}");

    {
        let test = 49;
        println!("Inside of these braces, the value of test is {test}");
    }

    println!("Notice that after the braces, the value of test is {test}");

    // Mutablity in action
    // Notice how for this case, we only need to use the "let" keyword a second time
    // when the variable is not declared as mutable

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Value of spaces in the first example is {spaces}");

    // Commented out example below won't work
    // Mutable variables don't allow you to change their types

    // let mut newSpaces = "   ";
    // newSpaces: usize = newSpaces.len();
    // println!("Value of newSpaces in the second example is {newSpaces}");
}