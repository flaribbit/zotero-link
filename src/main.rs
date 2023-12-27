use clipboard_win::{formats, get_clipboard, set_clipboard};

fn process(clip_text: String) -> String {
    clip_text.replace("(zotero://", "(https://www.zotero.org/")
}

fn main() {
    let mut last_clip_text = String::default();
    loop {
        let clip_text: String = get_clipboard(formats::Unicode).unwrap_or_default();
        if !clip_text.is_empty()
            && clip_text != last_clip_text
            && clip_text.find("(zotero://").is_some()
        {
            last_clip_text = process(clip_text);
            set_clipboard(formats::Unicode, &last_clip_text).unwrap();
            println!("Processed: {}", last_clip_text);
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
