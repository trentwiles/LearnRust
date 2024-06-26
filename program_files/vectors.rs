
fn main(){
    // vector of numbers
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{}", v[1]);


    let mut x: Vec<u32> = Vec::new();
    // this vector can be updated, since it is mutable
    x.push(123);

    println!("{}", x[0]);


    // accessing
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    // "match", an example of a control flow in rust
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    println!("{third}");

    // Avoiding errors when accessing invalid elements!
    let yea = vec![50, 60, 70];

    // let item1 = &yea[130];
    // ^^^^ cannot do this

    let item1 = yea.get(100);
    // Will either be a None or Some(&element)

    // here's another test with control flows
    let uhh: bool = false;
    match uhh {
        false => println!("It is false"),
        true => println!("Wrong, it's true")
    }

}