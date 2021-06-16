
fn private_add(x: u32, y: u32) -> u32 {
    x + y
}

pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

pub mod extra {
    pub fn square(x: u32) -> u32 {
        x * x
    }
}
