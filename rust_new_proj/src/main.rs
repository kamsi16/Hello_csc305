pub mod hold_user;
use hold_user::derived::user_defined;
use hold_user::primitive::compound;
use hold_user::primitive::scalar;

fn scalar_examples() {
    scalar::boolean();
    scalar::integer_example();
    scalar::textual_examples();
}


fn main() {
    println!("Hello, world!");
    user_defined::name();
    compound::age();
    scalar::height();
    scalar_examples();
}



