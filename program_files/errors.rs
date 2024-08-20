fn main(){
    use std::fs::File;
    use std::io::ErrorKind;

    //panic!("crash and burn");

    // when running rust with cargo, you can run a backtrace like so:
    // RUST_BACKTRACE=1 cargo run

    // Recoverable errors: errors where the program doesn't need to be stopped
    // This following code essentially functions as a try/catch
    let the_file_result = File::open("hello.txt");

    // based on if the program can open the file, "greeting_file" will
    // either contain the File, or the panic error
    let greeting_file = match the_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // you can take this a step further and get more specific with how errors
    // are handled, for instance if you want to do something different based on the error type
    let the_second_file_result = File::open("hello.txt");

    let second_file = match the_second_file_result {
        Ok(file) => file,
        // there's been an error, you can now go into what kind of error it is
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // not found? easy, just create the file
                Ok(fc) => fc,
                // can't create the file? second_file now equals the panic statement
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                // if there's another error, second_file is now this more generic panic statement
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // Panic shortcuts
    // Shorthand for opening a file, sends out a panic error if the file cannot be opened
    let third_file = File::open("hello.txt").unwrap();

    // Better way (and more production friendly)
    // The text inside of .expect is a note for programmers as to what the code should do
    let fourth_file = File::open("hello.txt")
        .expect("You need this file!");
}