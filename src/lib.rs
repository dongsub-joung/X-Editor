mod miyuki_user;
mod miyuki_import_core;
mod miyuki_core;
// use compile_type_eq::{assert_types_eq, assert_types_not_eq};

#[cfg(test)]
mod language_tests {
    use super::*;
    use crate::miyuki_user::Language;
    
    #[test]
    fn user_id_test() {
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
            Language::new(miyuki_user::LanguageOptions::JAPANESE);
        let lang_option= Language::new(
            miyuki_user::LanguageOptions::JAPANESE
        );

        // @TODO type compare
        // assert_types_eq::<i32, i32>();
    }
}

mod import_test{
    use super::*;
    use miyuki_import_core::*;
    
    #[test]
    fn import_test(){
        // No cookies.json case
        let sub= Future::from(x_api_rs::TwAPI::session::new());
        return import_x_data(sub);
    }
}

// User import
mod user_import_test{
    use crate::miyuki_core::*;

    #[test(name="Handle: Login_failed")]
    fn test1(){
        // No cookies.json case
        let sub= Future::from(x_api_rs::TwAPI::session::new());
        return import_x_data(sub);
    }
}