mod lib;
use lib::Clock;

fn main() {
    subtract_more_than_two_hours();
}

fn subtract_more_than_two_hours() {
    let clock = Clock::new(0, -400);
    println!("{:?}", clock);

}
