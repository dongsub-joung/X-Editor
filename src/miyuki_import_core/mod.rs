use crate::auth_x;

#[derive(Debug)]
struct ImportXApi;

impl ImportXApi{
    async fn verify_x_api(){
        // let cookies_path = PathBuf::from("cookies.json");
        // let mut user_session = x_api_rs::TwAPI::new(Some(cookies_path.clone()))?;

        if !user_session.is_logged_in().await? {
            user_session= auth_x::create_sesstion();
        }
    }
}
