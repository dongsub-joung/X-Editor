use thiserror::Error;
use suffix::SuffixTable;
use dotenvy::dotenv;
use std::{hash::Hash, path::PathBuf};
use x_api_rs::auth::SuspiciousLoginError;

trait Regular{
    fn new(str: String) -> Self;
    fn checking(&self) -> std::option::Option<String>;
}

struct ImporingXPost{
    x_url: String,
}

struct AuthRegular{
    checking_input: String,
}

impl Regular for AuthRegular{
    fn new(checking_input: String) -> Self {
        AuthRegular { checking_input }
    }

    fn checking(&self) -> std::option::Option<String> {
        // self.checking_input;
        // @TODO logic yet
        Some(String::new())
    }
}

#[derive(Debug)]
pub struct Auth;

impl Auth{
    pub fn input_user_id(user_input: String) -> String {
        let mut checked: String;
        {
            let regular_struct: AuthRegular= Regular::new(user_input);
            let option_checked= regular_struct.checking(); 
            if option_checked == None{
                checked= String::new();
            }else{
                checked= option_checked.unwrap();
            }
        }
        checked
    }
}

#[derive(Debug)]
struct Session;
// get_session_id();

struct SessionVarify{
    saved_hashed_session_id: String,
}
impl SessionVarify{
    pub fn new(saved_session_id: String) -> Self{
        // @TODO Do hash and save string
        // let saved_hashed_session_id= self.do_hash_md5(saved_session_id);
        let saved_hashed_session_id= String::new();
        Self { saved_hashed_session_id }
    }

    // @TODO return type MD5
    pub fn do_hash_md5(&self, normal: String) -> String{
        // let return_value= MD5::from(normal);
        String::new()
    }

    // @TODO Sesstion domain -> get sessition id(hashed)
    pub fn varify_session_string(&self, hashed_session_domain_id: &String) -> String{
        // @TODO let hashed_session_domain= session_obj.get_session_id();
        
        let mut v_bool:Vec<bool>= Vec::new();
        for (i, out_c) in self.saved_hashed_session_id.chars().into_iter().enumerate(){
            for (j,nasty_c) in hashed_session_domain_id.chars().into_iter().enumerate(){
                if i == j {
                    if out_c == nasty_c{
                        v_bool.push(true);
                    }else{
                        v_bool.push(false);
                    }
                }
            }
        }
    }
}hashed_session_domain

#[derive(Debug)]
struct XApi;

pub impl XApi{
    pub async fn create_sesstion(mut user_session: x_api_rs::TwAPI, primitive_type_auth: Miyuki) -> Result<(), Box<dyn std::error::Error>> {
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
                let login_err: &SuspiciousLoginError;
                if let Some(error) = error.downcast_ref::<SuspiciousLoginError>(){
                    login_err= error;
                };
                let username= uti::input_user_id(primitive_type_auth);
                if let Ok(None)= api.login(&username, &password, "", Some(error.1.clone()))
                .await{
                    eprintln!("{:?}", login_err);
                };
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
    pub async fn import_x_data<F>(mut user_session: dyn Future<Output= x_api_rs::TwAPI>, primitive_type: miyuki_core::OneLine)
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
