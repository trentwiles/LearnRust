fn main(){
    use std::collections::HashMap;

    let mut cars: HashMap<String, u32> = HashMap::new();

    cars.insert(String::from("Volvo"), 30000);
    cars.insert(String::from("Honda"), 50000);


    // for loop
    for x in &cars{
        let car_name = x.0;
        let price = x.1;
        println!("Car name: {} - Car price: {}", car_name, price);
    }
    
    println!("=-==-=-=-=-==-==-=-=-=-==-=-=-=-=-=-=-=-=-=");

    // OR you can do this
    for (first, second) in &cars{
        println!("{first} - {second}");
    }

    // accessing a value
    let search_q = "Volvo".to_string();
    
    let price_of_volvo = cars.get(&search_q).copied().unwrap_or(0);
    println!("According to the search, the value of the {search_q} is a {price_of_volvo}");
    println!();

    // Ownership
    // With an example
    let first_insert = String::from("example");
    let second_insert = String::from("foo");
    let mut hp: HashMap<String, String> = HashMap::new();
    // right now first insert and second insert can be accessable
    println!("For testing, here's first_insert and second_insert: {}, {}", first_insert, second_insert);
    hp.insert(first_insert, second_insert);

    // now first_insert and second_insert are not usable now
    //println!("{} - {}", first_insert, second_insert);
    // This won't work due to ownership issues
    // 41 |     println!("{} - {}", first_insert, second_insert);
   //     |                         ^^^^^^^^^^^^ value borrowed here after move

   // Overwrite
   let mut hp2: HashMap<String, u32> =  HashMap::new();
   hp2.insert("hey".to_string(), 43);
   hp2.insert("hey".to_string(), 49);
   for (item, item2) in &hp2{
    println!("{item} - {item2}");
   }
   // BONUS - here's another way to print
   println!("{hp2:?}");
   // {"hey": 49}
   // ^^^^ above will only print out only the second insertion; the first one has been overwritten
}