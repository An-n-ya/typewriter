use web_sys::{HtmlAudioElement, wasm_bindgen::prelude::Closure};
use leptos::logging::{log, error};


pub fn play_test() {
    play("wang4");
}

pub fn play(path: &str) {
    let audio = HtmlAudioElement::new_with_src(&format!("/typewriter/media/{path}.mp3")).unwrap();
    let _ = audio.play().unwrap();
}
