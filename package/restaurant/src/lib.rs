
mod front_of_house;

pub use crate::front_of_house::hosting;

fn eat_at_the_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
