use uuid::Uuid;

pub enum LanguageOptions{
    JAPANESE,
    CHINESE,
    RUSSIAN,
    ENGLISH,
}

pub struct Language {
    // @TODO UTF16
    pub option: LanguageOptions,
}
impl Language {
    pub fn new(option: LanguageOptions) -> Self{
        Self { option }
    } 
}

pub struct User{
    pub uid: uuid::Uuid,
    pub language: Language,
    // uptime: utilities::UpTime, cant cal
}

impl User{
    pub fn new(language: Language) -> Self{
        let uid= Uuid::new_v4();
        language.option;
        self { uid, language }
    }

    pub fn get_user_uid(&self) -> &uuid::Uuid{
        &self.uid
    }

    pub fn get_user_language_option(&self) -> &Language{
        &self.language
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

