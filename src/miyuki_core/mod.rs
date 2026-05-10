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
