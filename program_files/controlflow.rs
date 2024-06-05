fn main(){

    // if statments
    let x = 39;
    if x > 3{
        println!("{x} is bigger than 3!");
    }else{
        println!("{x} isn't bigger than 3!")
    }

    // if and elseif
    let new = "hey";
    if new.len() == 0{
        println!("New is small");
    }else if new.len() == 1{
        println!("New is medium sized");
    }else if new.len() == 2{
        println!("New is large");
    }else{
        println!("New is HUGE");
    }

    // if in a let statment
    // val will be 5 if true, 3 if false
    let val: u32 = if new.len() > 0 { 5 } else { 3 };
    println!("val is {val}");
    // You CANNOT do this:
    // let val1: u32 = if new.len() > 0 { 5 } else {"hey"};

    let val2: bool = x < 3;
    println!("val2 = {val2}");



    // loops
    // This is commented out so it doesn't run forever, but this is a "while true" loop
    // loop {
    //     println!("again!");
    // }

    let mut counter = 1;
    loop{
        if counter == 10{
            break;
        }
        println!("Looped {counter} times");
        counter += 1;
    }

    // returning a value from a loop
    let mut counter2 = 0;

    let result = loop {
        counter2 += 1;

        if counter2 == 10 {
            // when this happens, the value of counter2 will be 10
            // "break" is essentially used here as a return statement, hence 10 * 2 will be returned
            break counter2 * 2;
        }
    };
    println!("Value of that loop was {result}");


    // multiple loops and "loop labels"
    // this gets a bit complex...
    let mut central_counter = 0;
    'first_loop: loop{
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // breaks this inside loop
                break;
            }
            if central_counter == 2 {
                // breaks outer loop
                break 'first_loop;
            }
            remaining -= 1;
        }

        central_counter += 1;
    }

    // while
    let mut y = 0;
    while y < 10{
        println!("y = {y}");
        y += 1;
    }

    // "for" loop
    let arr = [1, 4, 5, 9, 0];
    let mut index = 0;
    while index < arr.len(){
        println!("{}", arr[index]);
        index += 1;
    }

    // fancy for loop
    for elem in arr{
        println!("{}", elem);
    }

    println!("-----------------------------------------");
    // looping through incriment
    // prints 3 2 1
    // last number in range is not inclusive
    for num in (1..4).rev(){
        println!("{}", num);
    }
}