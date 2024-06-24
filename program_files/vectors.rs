
fn main(){
    // vector of numbers
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{}", v[1]);


    let mut x: Vec<u32> = Vec::new();
    // this vector can be updated, since it is mutable
    x.push(123);

    println!("{}", x[0]);
}