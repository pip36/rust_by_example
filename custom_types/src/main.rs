use std::fs::File;

fn main() {
    // A standard struct
    struct Pie {
        filling: String,
        size: u8,
    }

    // A unit struct
    struct Blah;

    // A tuple struct
    struct Point(u32, u32);

    // structs can be fields of other structs
    struct Circle {
        center: Point,
        radius: u32,
    }

    let _chicken_pie = Pie {
        filling: String::from("chicken"),
        size: 3,
    };

    let _small_ball = Circle {
        center: Point(5, 5),
        radius: 1,
    };

    let _large_ball = Circle {
        radius: 9,
        .._small_ball // use the fields from small_ball
    };

    // ******************************************************** //
    // Enums
    // enums store different variants
    enum Monster {
        Bat,                                   // unit
        ColoredSlime(String),                  // tuple
        BigBoss { health: u64, name: String }, // stuct like
    }

    // enums can be pattern matched!
    fn view_monster(monster: &Monster) {
        match monster {
            Monster::Bat => println!("I'm a bat!"),
            Monster::ColoredSlime(color) => println!("I'm a {} slime!", color),
            Monster::BigBoss { health, name } => {
                println!("I'm {} and I have {} health!", name, health)
            }
        }
    }

    impl Monster {
        fn speak(&self) {
            match self {
                Self::Bat => println!("Squeek!"),
                Self::ColoredSlime(_) => println!("Splat!"),
                Self::BigBoss { name: _, health: _ } => {
                    println!("Glub!")
                }
            }
        }
    }

    let a_bat = Monster::Bat;
    let red_slime = Monster::ColoredSlime(String::from("red"));
    let super_shark = Monster::BigBoss {
        health: 100,
        name: String::from("Super Shark"),
    };

    // function pattern matching enum
    view_monster(&a_bat);
    view_monster(&red_slime);
    view_monster(&super_shark);

    // using implementation block on enum
    a_bat.speak();
    red_slime.speak();
    super_shark.speak();

    // C like enums example
    enum FileType {
        CSV = 1,
        XLS = 2,
        TXT = 3,
    }
    println!("CSV === {}", FileType::CSV as i32);
    println!("XLS === {}", FileType::XLS as i32);
    println!("TXT === {}", FileType::TXT as i32);

    // ************************************************ //
    // CONSTANTS
    const _DAYS_IN_WEEK: i8 = 7; // cannot be reassigned
}
