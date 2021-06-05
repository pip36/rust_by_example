fn main() {
    let num = 50;
    if num < 50 {
        println!("Less than 50.");
    } else if num > 50 {
        println!("Larger than 50.");
    } else {
        println!("Equal to 50.");
    }

    /* Since branches are just expressions they can be used for assignment */
    let is_fizzbuzz = if num % 5 == 0 && num % 3 == 0 {
        println!("It's a fizzbuzz");
        true
    } else {
        false
    };
    println!("{}", is_fizzbuzz);

    // ****************************************** //
    // Loops
    let mut counter = 0;
    loop {
        // an infinite loop
        counter += 1;
        if counter > 5 {
            println!("Loop done.");
            break;
        }
    }

    // Nested loops
    'outside: loop {
        'inside: loop {
            // break; // This would break the inside loop only
            break 'outside;
        }
    }

    // loop as an expression
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 5 {
            break counter; // return value of counter from loop
        }
    };
    println!("{}", result);

    // while loops
    let mut exit = false;
    while !exit {
        exit = true;
    }

    // for in loops

    /* Iterate a 'range'*/
    for n in 1..3 {
        println!("{}", n)
    }
    /* An inclusive range */
    for n in 1..=3 {
        println!("{}", n)
    }
    /* Any iterable thing will work */
    let mut fruits = vec!["apple", "melon", "plum"];
    for fruit in fruits {
        // This is implicitly the same as fruits.into_iter()
        // fruits has now been 'moved' so will be freed and unuavailable after the loop
        println!("{}", fruit);
    }
    // fruits.push("ok") Fails

    let mut fruits = vec!["apple", "melon", "plum"];
    for fruit in fruits.iter() {
        // .iter() will cause each fruit to be 'borrowed' inside the loop
        println!("{}", fruit);
    }
    fruits.push("banana");

    let mut fruits = vec!["apple", "melon", "plum"];
    for fruit in fruits.iter_mut() {
        // .iter_mut() borrows each fruit and makes it mutable.
        // allowing in place modifications
        *fruit = "eaten";
        println!("{}", fruit);
    }
    fruits.push("banana");

    // ****************************************** //
    // Pattern Matching
    // simple
    let is_easy = true;
    match is_easy {
        true => println!("It's easy!"),
        false => println!("It's hard..."),
    }

    // match can contain several values
    let num = 10;
    match num {
        1 => println!("It's 1."),
        2 | 3 => println!("It's a 2 or a 3"),
        4..=7 => println!("It's between 4 and 7 inclusive"),
        _ => println!("It's something else"),
    }

    // Lets do fizzbuzz by matching some expressions
    for n in 1..=100 {
        match (n, n % 3 == 0, n % 5 == 0) {
            (n, true, false) => println!("{}: Fizz", n),
            (n, false, true) => println!("{}: Buzz", n),
            (n, true, true) => println!("{}: FizzBuzz", n),
            _ => (),
        }
    }

    // structs can be destructured in match statements
    struct Person {
        name: String,
        is_fred: bool,
    }
    let fred = Person {
        name: String::from("fred jones"),
        is_fred: true,
    };
    match fred {
        Person {
            is_fred: true,
            name,
        } => println!("I'm {}", name),
        Person { is_fred: false, .. } => println!("I'm not fred"),
    }

    // guards can add conditions to each arm of the match
    let scores = (3, 5);
    match scores {
        (0, _) => println!("first invalid"),
        (_, 0) => println!("second invalid"),
        (x, y) if x == y => println!("Draw"),
        (x, y) if x > y => println!("First wins"),
        (x, y) if x < y => println!("Second wins"),
        _ => (), // this is necessary to help the compiler, even though it will never run
    }

    // Specific values in ranges can be bound, so they can be used in the match arm
    fn get_num() -> u32 {
        7
    }
    match get_num() {
        n @ 1..=10 => println!("{}", n), // Exact value that matched is bound
        _ => print!("Something else..."),
    }

    // Enums can be matched. That includes Option:
    fn maybe_get_num() -> Option<u32> {
        Some(10)
    }
    match maybe_get_num() {
        Some(n @ 10) => println!("It's {}!", n), // use binding to match specific
        Some(n) => println!("It's a number - {}", n), // any other success
        None => println!("Couldn't get num"),    // match failure
    }

    // Match can be clunky in some circumstances but "if let" can be used as shorthand
    let maybe = Some(5);

    match maybe {
        Some(5) => println!("5"),
        _ => {}
    }

    if let Some(5) = maybe {
        println!("5")
    }

    // enum example
    enum Duration {
        Hour,
        Week,
        Month(u8),
    }
    let h = Duration::Hour;
    let w = Duration::Week;
    let feb = Duration::Month(28);

    if let Duration::Hour = h {
        println!("hour");
    }

    if let Duration::Week = w {
        println!("week");
    }

    if let Duration::Month(days) = feb {
        println!("Month - {} days", days);
    }

    if let Duration::Month(days @ 28) = feb {
        println!("probably Feb");
    }

    // There is also while let which will continue to loop until the destructure fails
    let mut counter = Some(0);
    while let Some(i) = counter {
        if i < 10 {
            println!("Counter: {}", i);
            counter = Some(i + 1)
        } else {
            counter = None
        }
    }
}
