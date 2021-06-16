pub mod nested_module;
mod private_module;

pub fn exists() -> bool {
    private_module::boop();
    true
}

pub mod inline_public {
    pub fn woop() {
        ()
    }
}
