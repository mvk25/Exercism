use clock::Clock;

fn main() {
    subtract_more_than_two_hours();
}

fn subtract_more_than_two_hours() {
    let clock = Clock::new(0, 0).add_minutes(-160);

    assert_eq!(clock.to_string(), "21:20");
}
