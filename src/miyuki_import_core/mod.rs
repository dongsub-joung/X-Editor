use crate::auth_x;

#[derive(Debug)]
struct ImportXApi;

impl ImportXApi{
    async fn import_x_posts(mut user_session: x_api_rs::TwAPI, primitive_type: Miyuki){
        if !user_session.is_logged_in().await? { // @TODO if is_logged_in() consum lifetime, add &
            user_session= auth_x::create_sesstion(user_session, primitive_type);
        }

        /*
         * let user_id = api.me_rest_id().await?;
         * _let res = api.get_followingids(user_id.to_string(), -1).await?;
         *    tracing::debug!("response: {res:?}");
         *    let ids = res
         *    .entries
         *    .iter()
         *    .map(|v| v.as_i64().unwrap_or_default().to_string())
         *    .collect();
         *    let res = api.users_lookup(ids).await?;
         *    tracing::debug!("response: {res:?}");
         */

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
