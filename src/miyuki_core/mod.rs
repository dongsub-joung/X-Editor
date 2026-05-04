#[derive(Debug)]
pub struct Auth;

impl Auth{
    pub fn input_user_id(_str: &'static str) -> String {
        _str.to_string()
    }
}
