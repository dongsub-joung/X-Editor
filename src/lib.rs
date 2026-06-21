mod miyuki_user;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let user= miyuki_user::User::new(LanguageOptions::JAPANESE);
        let user_uid= user.get_user_data();

        assert_eq!(user_uid, user.uid);
    }

    #[test]
    fn language(){
        let user= User::new(LanguageOptions::JAPANESE);

        assert_eq!(LanguageOptions::JAPANESE, user.language);
    }
}
