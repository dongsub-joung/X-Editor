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
}

impl DocumentaryReport{
    pub fn get_doc_report(user: User){
        self { user: user.get_user_data(), up_time: user.get_up_time() }
    }
}

#[deriven(Debug)]
struct Fillter;
impl Fillter{
    pub fn string_check(string: String) -> Some(String){
        // @TODO validator
        // https://github.com/Keats/validator
        // https://github.com/seanmonstar/warp
    }
}

trait Regular{
    from(string: String) -> Self,
}

struct ImporingXPost{
    x_url: String,
}

impl ImportingXPost for Regular{
    pub fn from(string: String) -> Self{
        self { x_url: string }
    }
}


// @TODO for searching this file as MD5 hash by using bot
// @TODO this will send data to hacker as SMTP
struct VarifySaveFile{
    file_hash: hash::MD5,
}

impl VarifySaveFile{
    pub fn new() -> Self{
        let file_hash= hash::MD5::new();
        self { file_hash }
    }
}

pub enum utilities{
    Fillter: Fillter,
    Uptime: UpTime,
    varify_save_file: VarifySaveFile,
}
