use formplot::{generate, CSV};

fn main() {
    generate(&CSV::from("testbed/assets/google.csv")).unwrap();
}