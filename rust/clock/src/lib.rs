use std::fmt;

#[allow(dead_code)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

#[allow(dead_code)]
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
