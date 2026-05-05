// @TODO Miyiki_core is GUI thread

// @TODO Auth used on other btn option body
#[derive(Debug)]
pub struct Auth;

impl Auth{
//     @TODO
    pub fn input_user_id(_str: String) -> String {
        _str.to_string()
    }
}

// @TODO GUI
// ...

pub struct Caputre{
    pub Importing: marcro!(import_x_post),
    // ...
}

impl Caputre{
    pub fn caputring(&self, &mut miyuki: GUI) -> Result<Ok(), Err()>{
        loop{
            let inputed= miyuki.read_user_input(); // -> GUI struct
            match inputed.get_text().map_macro_type(){
                String::from("!import") => {
                    Caputre::inporting
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
