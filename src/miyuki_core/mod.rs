use thiserror::Error;
use suffix::SuffixTable;


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
#[derive(Debug)]
struct FailSaveUserCach;

#[derive(Error, Debug)]
pub enum ImportErrs{
    #[error("[FAILED] Geting user id")]
    FailGetUserId,
    #[error("[FAILED] Geting followers")]
    FailGetFollowing,
    #[error("[FAILED] looking user up")]
    FailUsersLookup,
    #[error("[FAILED] finding friends]")]
    FailGetFriends,
    #[error("[FAILED] finding Saved User Cach]")]
    FailSaveUserCach,
}

impl ImportXApi{
    pub async fn import_x_data<F>(mut user_session: F, primitive_type: miyuki)
    where
        F: Future<Output= x_api_rs::TwAPI>
    // -> Result<Box<dyn ImportErrs>
    {
        if !user_session.is_logged_in().await{
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

        let profile: miyuki::cach::User= make_user_profile_for_describtion(
            v_user_lookup_data, // info data
            res, // following 
            pagination, //friends
        );

        let _= match miyuki::Cach::save_cach_data(&profile){
            Ok(rsp) => tracing::debug!("saved cach user data"),
            Err(e) => {
                return ImportErrs::FailSaveUserCach;
            }
        }; 

        let user_profile_form: miyuki::Form= generate_user_profile_info(&profile);

        // miyuki_core::Randering::generate_user_profile_info(user_profile_form); 
    }
}



// @TODO Miyiki_core is GUI thread

// @TODO Auth used on other btn option body
#[derive(Debug)]
pub struct Auth;

impl Auth{
    pub fn input_user_id(user_inpute: String) -> String {
        let mut checked= utilities::Utilities::Regular::check(user_inpute); 
        let if !Some(_string) = checked{
            checked= Default::default();
        }

        checked
    }
}

struct OneLine{
    line: HashMap<Miyuki_GUI_Componante, String>;
}

impl OneLine{
   pub fn new(line_data: GUI_line, body :String) -> Self{
       let mut map: HashMap<Miyuki_GUI_Componante, String>= HashMap::new(); 
       self { map.push( (key:line_data, value: body) )  }
   }

    pub fn get_data(one_line: &self, count: usize) -> &HashMap<Miyuki_GUI_Componante, String>{
        one_line.line.iter.Fillter(|e| e.get_key(count) );
    }
}

// @TODO GUI
// ...
// @TODO Image processing
// https://github.com/image-rs/image

// @TODO macro[import_x_post]
// https://github.com/greyblake/whatlang-rs/blob/master/src/core/detect.rs

// @TODO HTMP rendering
// https://github.com/lambda-fairy/maud

pub struct Caputre{
    pub Importing: marcro!(import_x_post),
    // ...
}

impl Caputre{
    pub fn caputring(miyuki: RefCell<Rc<GUI>>) -> Result<Ok(), Err()>{
        loop{
            let inputed= miyuki.read_user_input()?; // -> GUI struct
            
            let accepted_inpute= inputed.get_text().map_macro_type();

            let regulared_text= utilities::Utilities::Fillter::string_check(accepted_inpute); 
            let regulared_text= match regulared_text{
                Some(string) => string,
                None => String::new()
            }

            let text_struct= Regular::from(regulared_text);
            
            // @TODO use OneLine

            match text_struct {
                Regular::Import => {
                    let data= text_struct.get_content();
                    Caputre::inporting // inporting!(data);
                },
                _ => {
                    Caputre::red_marking
                    Caputre::suggesting
                },
            }
        }

        thread::exit(0);
    }
}

