use std::fmt;
// #[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_hours = hours;
        let mut new_minutes = minutes;
        
        while new_minutes >= 60 {
            new_hours += new_minutes / 60;
            new_minutes %= 60;
        }
    
        while new_minutes < 0 {
            new_hours -= 1;
            new_minutes += 60;
        }

        while new_hours < 0 {
            new_hours += 24;
        }

        while new_hours >= 24 {
            new_hours -= 24;
        }

        
        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
    
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
       
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let mut new_hours = self.hours;
        write!(f, "{:02}:{:02}", self.hours , self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool { 
        self.hours == other.hours  && self.minutes == other.minutes
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self)
    }
}

// use std::fmt;
// // use std::string;

// const MINUTES_A_DAY: i32 = 60 * 24;

// #[derive(Debug)]
// pub struct Clock {
//     minutes: i32,
// }

// impl Clock {
//     pub fn new(hour: i32, minute: i32) -> Clock {
//         Clock {minutes: Clock::normalize_minutes(hour * 60 + minute)}
//     }

//     pub fn add_minutes(&mut self, minutes: i32) -> Clock {
//         Clock {minutes: Clock::normalize_minutes(self.minutes + minutes)}
//     }

//     fn normalize_minutes(minutes: i32) -> i32 {
//         let mut normalized = minutes;
//         while normalized < 0 {
//             normalized += MINUTES_A_DAY;
//         }
//         normalized % MINUTES_A_DAY
//     }
// }

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
//     }
// }

// impl PartialEq for Clock {
//     fn eq(&self, other: &Clock) -> bool {
//         self.minutes == other.minutes
//     }
// }