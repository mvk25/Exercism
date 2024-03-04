use time::PrimitiveDateTime as DateTime;
use time::Duration;



fn main() {
    fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
        use time::{Date, Time};
    
        DateTime::new(
            Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
            Time::from_hms(hour, minute, second).unwrap(),
        )
    }

    let start_date = dt(2011, 4, 25, 0, 0, 0);
    let added_time = start_date + Duration::hours(1000000000 / 3600);
    let expected_date = dt(2043, 1, 1, 1, 46, 40);

    println!("{}", start_date);
    println!("{}", expected_date);
    println!("{}", added_time);

}