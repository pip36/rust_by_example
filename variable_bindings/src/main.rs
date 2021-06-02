fn main() {
    let a_number = 100; // type can be inferred

    let mut mutable_number = a_number;
    mutable_number += 1; // "mut" must be used to explicitly make a variable mutable
    println!("{}", mutable_number);

    // variables are scoped to blocks

    let outer = 1;

    {
        let inner = 2;
        println!("{} {}", outer, inner)
    }
    // println!("{}", inner) - fails, out of scope

    // The same variable name can be "shadowed", reusing the same name.
    // useful when transforming an object into a different representation.
    let cake = "chocolate";
    struct Cake {
        flavour: String,
        tiers: u8,
    }
    let cake = Cake {
        flavour: String::from(cake),
        tiers: 2,
    };

    // shadowing affects mutability!
    let mut a_value = 1;

    {
        let a_value = 2; // shadowing in this inner scope

        // a_value += 1; value is no longer mutable. (it's "frozen")
    }

    a_value += 1; // value is mutable in this scope
}
