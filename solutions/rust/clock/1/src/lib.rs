#[derive(Debug, PartialEq, Eq)]
pub struct Clock{
    total_minutes: i32
}
use std::fmt;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let minutes_in_day = 24 * 60;

        let mut total_minutes = (hours * 60 + minutes) % minutes_in_day;

        if total_minutes < 0 {
            total_minutes += minutes_in_day;
            
        }

        Clock{
             total_minutes: total_minutes,
        }
        
        
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.total_minutes + minutes) // calls the self variable and add minutes
    }

}
    impl fmt::Display for Clock{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{

            let hours = self.total_minutes / 60;
            let minutes = self.total_minutes % 60;

            write!(f, "{hours:02}:{minutes:02}") // fill missing space with 0, use atleast 2 chars
        }
    }