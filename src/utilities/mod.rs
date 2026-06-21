// use chrono::{DateTime, Utc};

// #[derive(Debug)]
// struct UpTime{
//     // up_time: DataTime<Utc>,
//     // date: Date, @TODO if date no needed for calculating Time, delete it
// }

// impl UpTime{
//     pub fn new() -> Self{
//         let up_time = Utc::now();;

//         self { up_time }
//     }
    
//     pub fn get_now_time(&self) -> DateTime<Utc> {
//         Utc::now()
//     }

//     // per a sec, reset 
//     pub fn get_up_time(&self) -> &DateTime::datetime::NaiveTime{
//         // self.up_time.datetime.time;  //NaiveTime   (secs: u32,frac: u32)
//         let now= Utc::now();
//     }
// }



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
    fn from_str(string: String) -> Self;
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
    pub fn new(file: file::File) -> Self{
        let file_hash= hash::check_sum::new(file);
        self { file_hash }
    }

    // For Excention 
    // pub fn compare(check_shum: hash::MD5) -> bool{}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}