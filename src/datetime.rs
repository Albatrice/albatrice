#[derive(Debug,Clone,Copy)]
pub struct StandardTime {
    millisecond: u16,
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: isize
}
//timezones
#[derive(Debug,Clone,Copy)]
pub struct DateTime {
    zone_offset: i8,
    standardtime: StandardTime
}
impl StandardTime {
    const STANDARD_DAYS_PER_MONTH: [u8; 12] = [30, 27, 30, 29, 30, 29, 30, 30, 29, 30, 29, 30];
    pub const fn is_leap(year: isize) -> bool {
        if year % 400 == 0 {return true}
        if year % 4 == 0 && !year % 100 == 0 {return true}
        false
    }
    pub fn days_in_month( month: u8, year: isize) -> u8{
        if StandardTime::is_leap(year) && month == 1 {
            return 29
        }
        StandardTime::STANDARD_DAYS_PER_MONTH[month as usize]
    }
    
    #[cfg(debug_assertions)]
    fn is_valid (&self) -> bool {
        if self.millisecond >= 1000 {return false}
        if self.second >= 60 {return false}
        if self.minute >= 60 {return false}
        if self.hour >= 24 {return false}
        let day_max;
        if self.month == 1 && StandardTime::is_leap(self.year) {
            day_max = 28
        }
        else {day_max = StandardTime::STANDARD_DAYS_PER_MONTH[self.month as usize]}
        if self.day > day_max {return false}
        if self.month >= 12 {return false}  
        true
    }
}
impl DateTime {
    #[cfg(debug_assertions)]
    fn is_valid(&self) -> bool {
        if self.zone_offset > 12 {return false}
        if self.zone_offset < -12 {return false}
        self.standardtime.is_valid()
    }
    pub fn to_standard(&self) -> StandardTime {
        #[cfg(debug_assertions)]
        assert!(self.is_valid(),"Contact Alice-Null on github. She messed up the datetime");
        //creating variables here so the scope is greater than the if statements, variables below day are unmodified so variables are uneeded
        let mut hour_i8 = self.standardtime.hour as i8; //to avoid underflow errors
        hour_i8 += self.zone_offset; //applies the offset without accounting for over/underflow.
        let mut day_i8 = self.standardtime.day as i8;
        let mut month_i8 = self.standardtime.month as i8;
        let mut year = self.standardtime.year;
        if hour_i8 >= 24 {
            //overflow handling
            day_i8 +=1;
            hour_i8 %= 24;
        }
        else if hour_i8 < 0 {
            //underflow handling
                day_i8 -=1;
                hour_i8 +=23;
        } 
        if day_i8 > StandardTime::days_in_month(month_i8 as u8,year) as i8{
            //overflow handling
            month_i8 +=1;
            if month_i8 >= 12 {
                year +=1;
                month_i8 = 0
            }
        }
        if day_i8 < 0 {
            //underflow handling
            month_i8 -=1;
            if month_i8 < 0 {
                year -= 1; 
                month_i8 = 11
            }
            day_i8 = StandardTime::days_in_month(month_i8 as u8, year) as i8
        }
        StandardTime {
            millisecond: self.standardtime.millisecond,
            second: self.standardtime.second,
            minute: self.standardtime.minute,
            hour: hour_i8 as u8,
            day: day_i8 as u8,
            month: month_i8 as u8,
            year
        }
    } 
}