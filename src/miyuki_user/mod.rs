struct User{
    uid: UID,
    language: miyuki_core::Language, // UTF16
    uptime: utilities::UpTime,
}

impl User{
    pub fn new() -> Self{
        let uid= uid::Uid::new();
        let language= miyuki_core::Language::new(UTF16);
        let uptime= utilities::UpTime::new();
        self { uid, language, uptime }
    }

    pub fn get_user_data(&self) -> &Self{
        self
    }

    pub fn get_up_time(&self) -> utilities::UpTime{
        self.uptime.get_up_time()
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