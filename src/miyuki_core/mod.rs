use suffix::SuffixTable;

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

// @TODO GUI
// ...

pub struct Caputre{
    pub Importing: marcro!(import_x_post),
    // ...
}

impl Caputre{
    pub fn caputring(miyuki: RefCell<Rc<GUI>>) -> Result<Ok(), Err()>{
        loop{
            let inputed= miyuki.read_user_input()?; // -> GUI struct
            
            let accepted_inpute= inputed.get_text().map_macro_type();

            let regulared_text= utilities::Utilities::Regular::check(accepted_inpute); 
            let regulared_text= match regulared_text{
                Some(string) => string,
                None => String::new()
            }

            let text_struct= Regular::from(regulared_text);
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
