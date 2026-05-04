use crate::auth_x;

#[derive(Debug)]
struct ImportXApi;

impl ImportXApi{
    async fn verify_x_api(mut user_session: x_api_rs::TwAPI){
        if !user_session.is_logged_in().await? {
            user_session= auth_x::create_sesstion();
        }
    }
}
