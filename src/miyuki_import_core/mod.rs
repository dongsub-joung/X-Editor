use crate::auth_x::*;
use thiserror::Error;

#[derive(Debug)]
struct ImportXApi;

#[derive(Debug)]
struct FailGetUserId;

#[derive(Error, Debug)]
enum ImportErrs{
    #[error("[FAILED] Geting user id")]
    FailGetUserId(),
}

impl ImportXApi{
    async fn import_x_data(mut user_session: x_api_rs::TwAPI, primitive_type: Miyuki) ->Result<Ok(), ImportErrs>{
        if !user_session.is_logged_in().await? {
            user_session= auth_x::create_sesstion(user_session, primitive_type);
        }

        let user_id = match user_session.me_rest_id().await{
            Ok(res) => res,
            Err(e) => {
                return e;
            },
        };
        let res = user_session.get_followingids(user_id.to_string(), -1).await?;
        tracing::debug!("response: {res:?}");

        let ids = res
            .entries
            .iter()
            .map(|v| v.as_i64().unwrap_or_default().to_string())
            .collect();

        let res = user_session.users_lookup(ids).await?;
        tracing::debug!("response: {res:?}");

        // loop {
        //     let pagination = api.get_friends(user_id, true, Some(cursor.into()))?;
        //     cursor = pagination.cursor.clone();
        //     tracing::debug!("Found {:?} following", pagination.entries.len());
        //     if !pagination.has_more {
        //         break;
        //     }
        // }
    }
}
