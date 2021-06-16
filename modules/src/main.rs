mod folder; // This does the same but finds 'folder/mod.rs'
mod math; // This looks for a file named 'math.rs' and adds it's contents as a module

fn main() {
    let added = math::add(1, 2);
    let squared = math::extra::square(2);

    folder::exists();
    folder::nested_module::test();
    // folder::private_module::boop(); This is private!
    folder::inline_public::woop();

    println!("{}", added);
    println!("{}", squared);
}
