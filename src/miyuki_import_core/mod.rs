use crate::auth_x;
use thiserror::Error;
use std::future::Future;

#[derive(Debug)]
struct ImportXApi;

#[derive(Debug)]
struct FailGetUserId;
#[derive(Debug)]
struct FailGetFollowing;
#[derive(Debug)]
struct FailUsersLookup;
#[derive(Debug)]
struct FailGetFriends;

#[derive(Error, Debug)]
pub enum ImportErrs{
    #[error("[FAILED] Geting user id")]
    FailGetUserId,
    #[error("[FAILED] Geting followers")]
    FailGetFollowing,
    #[error("[FAILED] looking user up")]
    FailUsersLookup,
    #[error("[FAILED finding friends]")]
    FailGetFriends,
}

impl ImportXApi{
    pub async fn import_x_data<F>(mut user_session: F, primitive_type: Miyuki)
    where
        F: Future<Output= x_api_rs::TwAPI>,
    -> Result<Box<dyn ImportErrs>
    {

        if !user_session.is_logged_in().await? {
            user_session= auth_x::XApi::create_sesstion(user_session, primitive_type);
        }

        let user_id = match user_session.me_rest_id().await{
            Ok(res) => res,
            Err(e) => {
                return ImportErrs::FailGetUserId;
            },
        };

        let res = match user_session.get_following_ids(user_id.to_string(), -1).await{
            Ok(pagination_response) =>{ pagination_response },
            Err(e) => {
                return ImportErrs::FailGetFollowing;
            },
        };

        tracing::debug!("response: {res:?}");

        let ids = res
            .entries
            .iter()
            .map(|v| v.as_i64().unwrap_or_default().to_string())
            .collect();

        let v_user_lookup_data = match user_session.users_lookup(ids).await{
            Ok(v_data) => { v_data },
            Err(e) => {
                return ImportErrs::FailUsersLookup;
            }
        };
        tracing::debug!("response: {res:?}");

        let pagination; //@TODO
        loop {
             pagination = match api.get_friends(user_id, true, Some(cursor.into())){
                Ok(e) => { e },
                Err(e) =>{
                    return ImportErrs::FailGetFriends;
                },
             };
            
             cursor = pagination.cursor.clone();
             tracing::debug!("Found {:?} following", pagination.entries.len());
             
             if !pagination.has_more {
                 break;
             }
         }

        // struct User
        let profile= make_user_profile_for_describtion(
            v_user_lookup_data, // info data
            res, // flolowing 
            pagination, //friends
        );

        let user_profile_form:: Cach_User= generate_user_profile_info(profile);

        miyuki_core::Randering::generate_user_profile_info(user_profile_form); 
    }
}
