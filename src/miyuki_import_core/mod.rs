use crate::auth_x;

#[derive(Debug)]
struct ImportXApi;

impl ImportXApi{
    async fn verify_x_api(){
        let user_session;

        if !api.is_logged_in().await? {
            user_session= auth_x::create_sesstion();
        }
    }
}
