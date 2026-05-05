struct UpTime{
    date: Date, // @TODO if date no needed for calculating Time, delete it
    up_time: Time,
}

impl UpTime{
    pub fn new() -> Self{
        let data= date::Date::new();
        let up_time= time::Time::new();  // @TODO find a crate for Time

        self { data, up_time }
    }
    
    pub fn get_date(&self) -> Time {
        date::Date::get_now().fromat("%yyy%-%mm%-%dd%");
    }

    pub fn get_up_time(&self) -> &Self{
        // @TODO calculate Time

        self.up_time
    }
}

struct DocumentaryReport{
    user_id: UID,
    up_time: UpTime,
    
    
}

enum utilities{

}


impl DocumentaryReport{

}
