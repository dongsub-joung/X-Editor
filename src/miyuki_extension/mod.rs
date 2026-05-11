use clipboard_win::{Clipboard, formats, Getter, Setter};

// @TODO If Miyuki extension use system call, Antivirus will block Miyuki
// How can I do?
// https://x.com/jds_invoker/status/2053783852347425270

struct Clipboard{
    v_clip_string: Vec<String>
}

impl Clipboard{
    pub fn get_clipboard() -> Clipboard {
        let v_clip_string= // clipboard_win::get_clipboard_string()

        self { v_clip_string }
    }
}

// IDK TCP shadow or SMTP wrapper protocol?


