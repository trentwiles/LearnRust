fn main(){
    // classic Buller jolly rancher flavor example
    enum Flavors{
        BlueRaspberry,
        Watermellon,
        SourApple
    }

    struct JollyRancher{
        flavor: Flavors,
        eaten: bool,
        year: u32
    }

    let my_rancher = JollyRancher{
        flavor: Flavors::SourApple,
        eaten: false,
        year: 1993
    };

    println!("{}", my_rancher.year);
    println!("hey");


    // New Example
    enum Foods{
        Burger(u32),
        Fries(u32),
        Popsicle(String)
    }

    let my_fries = Foods::Fries(10);
    let popsicle = Foods::Popsicle(String::from("this is my popsicle"));

    // Enum with structs embeded
    enum Person{
        // embeded structs
        location {x: u32, y: u32},
        name(String),
        age(u32),
        first_initial(char),
        tuple_example(u32, u32, u32)
    }

    enum MultipleOptions<T>{
        variable(T)
    }

    let example1 = MultipleOptions::<String>::variable("hey".to_string());

    // JollyRancher control flow
    fn value_in_cents(jolly: Flavors) -> bool {
        match jolly {
            Flavors::BlueRaspberry => true,
            Flavors::Watermellon => false,
            Flavors::SourApple => {
                println!("eww i hate sour");
                println!("it is the worst");
                false
            }
        }
    }

    

}

