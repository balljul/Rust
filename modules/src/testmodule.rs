pub mod tests {
    pub mod messages{
        pub fn hello() {
            println!("Hello Compiler")
        }
    }
}

pub use tests::messages as test;
