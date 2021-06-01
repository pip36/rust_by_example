use std::fmt;

/// This is the documentation comment for the maBlah { field1: 5 }tion
/**
 - This is more docmentation
 - Docs can be generated via `cargo doc` command
**/
fn main() {
    // This is a single line comment...

    /*
      ...and this can be used for longer
      multiline comments
    */
    let output = "Hello world";

    let a_string = format!("{}", output);

    print!("{}", a_string); // Same as format! but print to stdout
    println!("{}", a_string); // Also append a newline
    eprint!("{}", a_string); // Print to stderr
    eprintln!("{}", a_string); // Also append a newline

    // ********************************************************* //
    // Printing complex objects
    #[allow(dead_code)]
    struct Blah {
        value: i32,
    }

    impl fmt::Display for Blah {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(Blah - {})", self.value)
        }
    }

    impl fmt::Debug for Blah {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(DEBUG BLAH {{ value: {} }})", self.value)
        }
    }

    let my_blah = Blah { value: 4 };

    println!(
        "{} won't print without implementing the Display trait",
        &my_blah
    );

    let s = my_blah.to_string(); // Display trait also adds to_string() method
    println!("{}", s);

    println!(
        "{:?} won't print without implementing the Debug trait",
        &my_blah
    );

    // ***************************************************************** //
    // Formatting Example
    let pi = 3.141592;
    println!("pi is roughly {0:.2}", pi); // specify 2 decimal places
}
