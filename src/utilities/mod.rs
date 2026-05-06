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

    pub fn get_up_time(&self) -> &Time{
        // @TODO calculate Time

        self.up_time
    }
}

struct DocumentaryReport{
    user: user::User,
    up_time: user::UpTime,    
}

impl DocumentaryReport{
    pub fn get_doc_report(){
        self { user: User::get_user_data, up_time: User::get_up_time }
    }
}

#[deriven(Debug)]
struct Filter;
impl Fillter{
    pub fn string_check(string: String) -> Some(String){
        // validator
    }
}

pub enum utilities{
    Fillter: Fillter
}
