fn main(){
    example();
    example2(48);
    example3();
}

fn example(){
    println!("Another function!");
}

fn example2(x: i32){
    println!("You passed in {x}");
}

fn example3() {
    // y is equal to the eval of the "block" (content inside of the brackets)
    let y = {
        let x = 3;
        x + 1;
        x - 10
    };

    println!("The value of y is: {y}");
}