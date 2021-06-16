use std::iter::Map;

fn main() {
    just_a_unit();
    let _n = add(3, 4);

    let s1 = Rectangle::unit();
    let s2 = Rectangle::new(5, 4);
    println!("s1 area - {}", s1.area());
    println!("s2 area -{}", s2.area());

    /*************************************************/
    // Closures
    //normal fn
    fn inc(n: i32) -> i32 {
        n + 1
    }
    // closure
    let inc = |n: i32| n + 1;

    // closure as a parameter
    let f_num = || 4;
    fn apply<F>(f: F)
    where
        F: Fn() -> u32,
    {
        f();
    }

    apply(f_num);
    apply(f_num);
    f_num();

    // Higher order functions + lazy iterator
    let num = (0..)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take_while(|&x| x < 100)
        .fold(0, |sum, x| sum + x);

    print!("{}, ", num)
}

fn just_a_unit() {}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    /* A couple of static methods */
    fn unit() -> Rectangle {
        Rectangle {
            height: 1,
            width: 1,
        }
    }
    fn new(height: u32, width: u32) -> Rectangle {
        Rectangle { height, width }
    }

    /* an instance method with access to the instance via 'self' */
    fn area(&self) -> u32 {
        self.height * self.width
    }
}
