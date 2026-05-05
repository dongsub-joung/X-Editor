mod miyuki_core;
mod miyuki_import_core;
mod auth_x;

fn main() {
    let miyuki= Rc<RefCell<miyuki_core::MiyukiGui::init()>>;

    loop{
        miyuki_core::capturing(miyuki);
    }
}
