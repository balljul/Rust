mod testmodule;
pub use testmodule::tests::messages as test;

fn main() {
    test::hello();
}
