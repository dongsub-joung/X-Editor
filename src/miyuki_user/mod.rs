enum LanguageOptions{
    JAPANESE,
    CHINESE,
    RUSSIAN,
    ENGLISH,
}

struct Language{
    // @TODO UTF16
    language: LanguageOptions,
}

struct User{
    pub uid: UID,
    language: Language,
    // uptime: utilities::UpTime, cant cal
}

impl User{
    pub fn new() -> Self{
        let uid= uid::Uid::new();
        let language= LanguageOptions::JAPANESE;
        self { uid, language}
    }

    pub fn get_user_data(&self) -> &Uid{
        self.uid
    }

    // pub fn get_up_time(&self) -> utilities::UpTime{
    //     self.uptime.get_up_time()
    // }
}

// struct DocumentaryReport{
//     user: User,
    
// }

// impl DocumentaryReport{
//     pub fn get_doc_report(user: User){
//         self { user: user.get_user_data() }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let user= User::new();
        let user_uid= user.get_user_data();

        assert_eq!(user_uid, user.uid);
    }
}