fn main() {
    // Most of the explicit type definitions are optional.

    let _signed_eight_bit: i8 = 127;
    let _signed_sixty_four_bit: i64 = -100_000;
    let _unsigned_eight_bit: u8 = 127;
    let _unsigned_sixty_four_bit: u64 = 100_000;

    let _float: f64 = 5.234;

    let _character: char = '👍';

    let _is_boolean: bool = true;

    let _unit = ();

    let _fruit_array: [&str; 3] = ["apple", "pear", "peach"];

    let _tuple: (&str, bool) = ("apple", true);
    _tuple.0; // "apple"
    _tuple.1; // true
    let (_name, _is_fruit) = _tuple; // extract via destructuring

    // variables must be explicitly defined as mutable
    let mut _counter = 0;
    _counter += 1;

    // ******************************************************** //
    // Arrays
    let _num_array = [1, 2, 3]; // a fixed size array
    let _repeat_array = [1; 100]; // an array of length 100, every element set to '1'

    _num_array[0]; // 1
}
