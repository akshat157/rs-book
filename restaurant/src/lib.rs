pub mod front_of_house; 
// Usage of a function from above module: crate::front_of_house::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

}

