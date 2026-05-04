use dotenvy::dotenv;
use std::path::PathBuf;
use x_api_rs::auth::SuspiciousLoginError;

use crate::miyuki_core;

#[derive(Debug)]
pub struct XApi;

impl CustomSuspiciousLoginError for x_api_rs::auth::SuspiciousLoginError {
    pub fn new() -> Self{
        let string= String::new();
        let flow=
    }
}

impl XApi{
    pub async fn create_sesstion(mut user_session: x_api_rs::TwAPI, primitive_type_auth: Miyuki) -> Result<(), Box<dyn std::error::Error>> {
//         @TODO
        tracing_subscriber::fmt::init();
        let _ = dotenv();
        let cookies_path = PathBuf::from("cookies.json");
        let username = std::env::var("USERNAME")?;
        let password = std::env::var("PASSWORD")?;
        tracing::debug!("username: {username}");
        tracing::debug!("password: {password}");

        let mut api = x_api_rs::TwAPI::new(Some(cookies_path.clone()))?;
        if !cookies_path.exists() {
            let result = api.login(&username, &password, "", None).await;
            if let Err(error) = result {
                let login_err: SuspiciousLoginError;
                if let Some(error) = error.downcast_ref::<SuspiciousLoginError>(){
                    login_err= error.unwrap();
                };
                let username= miyuki_core::Auth::input_user_id(primitive_type_auth);
                api.login(&username, &password, "", Some(error.1.clone()))
                .await?;
            }

            match api.save_session(){
                Err(e) => { eprintln!("{}", e); }
                _ => { dbg!("[OK] Session connected"); },
            };
        }

        // always call this for extract csrf
        let is_logged_in = api.is_logged_in().await?;
        tracing::info!("is logged: {is_logged_in}");

        Ok(())
    }
}
