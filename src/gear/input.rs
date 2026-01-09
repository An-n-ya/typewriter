use lazy_regex::regex;
use lazy_static::__Deref;
use leptos::{ev, leptos_dom::helpers::window_event_listener, logging::log, prelude::*};
use leptos_use::use_window_focus;

use crate::action::{動作, 動作給一參數, 動作給一參數得一結果};
use crate::key_code::{KeyCode, 網頁鍵值轉換};
use crate::spelling_algebra::拼寫運算;
use crate::變換;

pub fn 焦點事件處理機關(重置並擊狀態: impl 動作) {
    let 鍵盤輸入焦點源 = Selector::new(use_window_focus());
    Effect::new(move |_| {
        if 鍵盤輸入焦點源.selected(&false) {
            重置並擊狀態();
        }
    });
}

#[allow(dead_code)]
pub struct 檔位 {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

pub struct 觸鍵消息 {
    pub 鍵碼: KeyCode,
    pub 檔位: 檔位,
}

pub fn 輸入事件處理機關(
    處理功能鍵: impl 動作給一參數得一結果<觸鍵消息, bool>,
    既然落鍵: impl 動作給一參數<KeyCode>,
    既然抬鍵: impl 動作給一參數<KeyCode>,
) {
    let keydown_handle = window_event_listener(ev::keydown, move |ev| {
        log!("落鍵 key = {}, code = {}", &ev.key(), ev.code());
        let 鍵碼 = 網頁鍵值轉換(&ev.code());
        let 檔位 = 檔位 {
            shift: ev.shift_key(),
            ctrl: ev.ctrl_key(),
            alt: ev.alt_key(),
            meta: ev.meta_key(),
        };
        if 處理功能鍵(觸鍵消息 { 鍵碼, 檔位 }) {
            ev.prevent_default();
        }
        if 鍵碼 != KeyCode::No {
            既然落鍵(鍵碼);
        }
    });

    let keyup_handle = window_event_listener(ev::keyup, move |ev| {
        log!("抬鍵 key = {}, code = {}", &ev.key(), &ev.code());
        let 鍵碼 = 網頁鍵值轉換(&ev.code());
        if 鍵碼 != KeyCode::No {
            既然抬鍵(鍵碼);
        }
    });

    on_cleanup(move || {
        keydown_handle.remove();
        keyup_handle.remove();
    });
}

pub fn 轉寫輸入碼序列(原形: &str) -> String {
    let mut 運算結果 = 原形.to_owned();
    let 運算規則 = [
        // 音調
        變換!("(.+)a(.+)1$", "$1ā$2"),
        變換!("(.+)a(.+)2$", "$1á$2"),
        變換!("(.+)a(.+)3$", "$1ǎ$2"),
        變換!("(.+)a(.+)4$", "$1à$2"),
        // o 韻母
        變換!("(.+)o(.+)1$", "$1ō$2"),
        變換!("(.+)o(.+)2$", "$1ó$2"),
        變換!("(.+)o(.+)3$", "$1ǒ$2"),
        變換!("(.+)o(.+)4$", "$1ò$2"),
        // e 韻母
        變換!("(.+)e(.+)1$", "$1ē$2"),
        變換!("(.+)e(.+)2$", "$1é$2"),
        變換!("(.+)e(.+)3$", "$1ě$2"),
        變換!("(.+)e(.+)4$", "$1è$2"),
        // i 韻母
        變換!("(.+)i(.+)1$", "$1ī$2"),
        變換!("(.+)i(.+)2$", "$1í$2"),
        變換!("(.+)i(.+)3$", "$1ǐ$2"),
        變換!("(.+)i(.+)4$", "$1ì$2"),
        // u 韻母
        變換!("(.+)u(.+)1$", "$1ū$2"),
        變換!("(.+)u(.+)2$", "$1ú$2"),
        變換!("(.+)u(.+)3$", "$1ǔ$2"),
        變換!("(.+)u(.+)4$", "$1ù$2"),
        // ü 韻母
        變換!("(.+)ü(.+)1$", "$1ǖ$2"),
        變換!("(.+)ü(.+)2$", "$1ǘ$2"),
        變換!("(.+)ü(.+)3$", "$1ǚ$2"),
        變換!("(.+)ü(.+)4$", "$1ǜ$2"),
    ];
    for 運算 in 運算規則 {
        match 運算 {
            拼寫運算::變換 {
                ref 模式, 替換文字
            } => {
                運算結果 = 模式.replace_all(&運算結果, &*替換文字).to_string();
            }
            _ => {}
        };
    }
    運算結果
}
