use suffix::SuffixTable;

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
    pub fn caputring(miyuki: RefCell<Rc<GUI>>) -> Result<Ok(), Err()>{
        loop{
            let inputed= miyuki.read_user_input()?; // -> GUI struct
            
            let accepted_inpute.get_text().map_macro_type() ;

            let 

            match accepted_inpute {
                accepted_inpute.contains("!import") => {
                    let sa= SubffixTable::from_parts(accepted_inpute);
                    Caputre::inporting // inporting!(sa.positions("!import("));
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
