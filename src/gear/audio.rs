use web_sys::HtmlAudioElement;


pub fn play(path: &str) {
    let mut p = path.to_owned();
    if !p.chars().last().map_or(false, |c| matches!(c, '1' | '2' | '3' | '4')) {
        p.push('1'); 
    }
    let audio = HtmlAudioElement::new_with_src(&format!("/typewriter/media/{p}.mp3")).unwrap();
    let _ = audio.play().unwrap();
}
