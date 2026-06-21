mod miyuki_user;

#[cfg(test)]
mod tests {
    use crate::miyuki_user::Language;
    use super::*;
    use compile_type_eq::{assert_types_eq, assert_types_not_eq};

    #[test]
    fn it_works() {
        let user= miyuki_user::User::new(
            miyuki_user::Language::new(
                miyuki_user::LanguageOptions::JAPANESE
            )
        );
        let user_uid= user.get_user_uid();
        
        assert_eq!(user_uid, &user.uid);
    }
    
    #[test]
    fn language(){
        let user= miyuki_user::User::new(
            Language::new(miyuki_user::LanguageOptions::JAPANESE)
        );
        let lang_option= Language::new(
            miyuki_user::LanguageOptions::JAPANESE
        );

        // @TODO type compare
        // assert_types_eq::<i32, i32>();
    }
}
