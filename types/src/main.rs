use std::convert::TryFrom;

fn main() {
    // types can only be explicitly cast
    let decimal = 65.1234_f32; // "f32 suffix can specify the type"
    let integer = decimal as u8; // "65"
    let character = integer as char; // "A"

    println!("{},{},{}", decimal, integer, character);

    // types can be inferred even after instantiation
    let num = 5;
    let mut list = Vec::new();
    list.push(num); // The compiler can now infer that the above is a Vec<i32>

    // types can also be aliased for clarity or to reduce boilerplate
    type CarID = u32;
    struct Car {
        id: CarID,
        name: String,
    }

    // From and Into traits can provide conversion between types
    // A basic example
    let a_str = "hey";
    let a_string = String::from(a_str);

    // "From" trait can be implemented to convert between custom types
    #[derive(Debug)]
    struct Point(u32, u32);
    impl From<&str> for Point {
        fn from(s: &str) -> Self {
            let vals: Vec<&str> = s.split(",").collect();
            let x = vals[0].parse::<u32>();
            let y = vals[1].parse::<u32>();

            return Point(x.unwrap(), y.unwrap());
        }
    }
    let my_point = Point::from("0,4");
    let my_point_2: Point = "0,4".into(); // and you get "into()" for free!
    println!("{:?}", my_point);
    println!("{:?}", my_point_2);

    // "TryFrom" is also available if the conversion may fail
    // in this case a Result is returned

    impl TryFrom<String> for Point {
        type Error = ();

        fn try_from(s: String) -> Result<Self, Self::Error> {
            let vals: Vec<&str> = s.split(",").collect();
            let x = vals[0].parse::<u32>();
            let y = vals[0].parse::<u32>();

            if x.is_ok() && y.is_ok() {
                return Ok(Point(x.unwrap(), y.unwrap()));
            } else {
                Err(())
            }
        }
    }

    let result_1 = Point::try_from(String::from("blah"));

    if result_1.is_err() {
        println!("OOPS result1 failed");
    }
}
