use crate::game::level::{DifficultyTier, FieldSize};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MenuStrings {
    pub title: String,
    pub subtitle: String,
    pub play_game: String,
    pub level_select: String,
    pub sound_on: String,
    pub sound_off: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LevelSelectStrings {
    pub title: String,
    pub back: String,
    pub size_small: String,
    pub size_medium: String,
    pub size_big: String,
    pub diff_relaxed: String,
    pub diff_challenging: String,
    pub diff_hard: String,
    pub generating_levels: String,
}

impl LevelSelectStrings {
    pub fn size_label(&self, size: FieldSize) -> &str {
        match size {
            FieldSize::Small6x6 => &self.size_small,
            FieldSize::Medium8x8 => &self.size_medium,
            FieldSize::Big10x10 => &self.size_big,
        }
    }

    pub fn difficulty_label(&self, diff: DifficultyTier) -> &str {
        match diff {
            DifficultyTier::Relaxed => &self.diff_relaxed,
            DifficultyTier::Challenging => &self.diff_challenging,
            DifficultyTier::Hard => &self.diff_hard,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HudStrings {
    pub level_template: String,
    pub moves: String,
    pub par: String,
    pub stats_template: String,
}

impl HudStrings {
    pub fn format_level(&self, id: u32) -> String {
        self.level_template.replace("{id}", &id.to_string())
    }

    pub fn format_stats(&self, moves: u32, par: u32) -> String {
        self.stats_template
            .replace("{moves}", &moves.to_string())
            .replace("{par}", &par.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WinModalStrings {
    pub title: String,
    pub completed_template: String,
    pub eval_perfect: String,
    pub eval_great: String,
    pub eval_good: String,
    pub next_level: String,
    pub replay: String,
    pub menu: String,
}

impl WinModalStrings {
    pub fn format_completed(&self, moves: u32, par: u32) -> String {
        self.completed_template
            .replace("{moves}", &moves.to_string())
            .replace("{par}", &par.to_string())
    }

    pub fn rating_evaluation(&self, moves: u32, par: u32) -> &str {
        if moves <= par {
            &self.eval_perfect
        } else if moves <= par + (par / 2).max(2) {
            &self.eval_great
        } else {
            &self.eval_good
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LocaleStrings {
    pub locale: String,
    pub language_name: String,
    pub menu: MenuStrings,
    pub level_select: LevelSelectStrings,
    pub hud: HudStrings,
    pub win_modal: WinModalStrings,
}

pub const LOCALES_RAW_JSON: &[&str] = &[
    include_str!("../../assets/locales/en-US.json"),
    include_str!("../../assets/locales/ru-RU.json"),
    include_str!("../../assets/locales/es-ES.json"),
    include_str!("../../assets/locales/de-DE.json"),
    include_str!("../../assets/locales/fr-FR.json"),
    include_str!("../../assets/locales/zh-CN.json"),
    include_str!("../../assets/locales/ko-KR.json"),
    include_str!("../../assets/locales/ja-JP.json"),
];

static LOCALES: OnceLock<Vec<LocaleStrings>> = OnceLock::new();

fn load_all_locales() -> Vec<LocaleStrings> {
    LOCALES_RAW_JSON
        .iter()
        .map(|raw| serde_json::from_str(raw).expect("Failed to deserialize embedded locale JSON"))
        .collect()
}

pub fn get_locales_list() -> &'static [LocaleStrings] {
    LOCALES.get_or_init(load_all_locales).as_slice()
}

/// Normalizes locale tags with various separators ('-', '_', '+') to lowercased hyphenated format.
/// e.g. "pt_BR", "pt+BR", "PT-BR" -> "pt-br"
pub fn normalize_locale_tag(tag: &str) -> String {
    tag.trim().replace(['_', '+'], "-").to_ascii_lowercase()
}

/// Resolves a requested locale tag against available translations.
/// 1. Exact normalized match (e.g. "ru-ru" matches "ru-RU", "ja-jp" matches "ja-JP")
/// 2. Shorthand alias match (e.g. "jp" -> "ja", "cn" -> "zh", "kr" -> "ko")
/// 3. Language prefix match (e.g. "ru" or "ru-kz" matches "ru-RU", "ja" matches "ja-JP", "zh-hans" matches "zh-CN")
/// 4. Fallback to default English ("en-US")
pub fn resolve_locale(tag: &str) -> &'static LocaleStrings {
    let locales = get_locales_list();
    let norm = normalize_locale_tag(tag);
    let raw_prefix = norm.split('-').next().unwrap_or("");
    let base_lang = match raw_prefix {
        "jp" => "ja",
        "cn" => "zh",
        "kr" => "ko",
        other => other,
    };

    locales
        .iter()
        .find(|l| normalize_locale_tag(&l.locale) == norm)
        .or_else(|| {
            (!base_lang.is_empty()).then(|| {
                locales.iter().find(|l| {
                    normalize_locale_tag(&l.locale)
                        .split('-')
                        .next()
                        .is_some_and(|prefix| prefix == base_lang)
                })
            })?
        })
        .unwrap_or(&locales[0])
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
extern "C" {
    fn parking_get_system_language() -> i32;
}

#[cfg(target_arch = "wasm32")]
pub fn detect_locale_tag() -> String {
    match unsafe { parking_get_system_language() } {
        1 => "ru-RU".to_string(),
        2 => "es-ES".to_string(),
        3 => "de-DE".to_string(),
        4 => "fr-FR".to_string(),
        5 => "ja-JP".to_string(),
        6 => "zh-CN".to_string(),
        7 => "ko-KR".to_string(),
        _ => "en-US".to_string(),
    }
}

#[cfg(target_os = "android")]
fn query_android_jni_locale() -> Option<String> {
    unsafe {
        let env = macroquad::miniquad::native::android::attach_jni_env();
        if env.is_null() {
            return None;
        }

        let find_class = (**env).FindClass?;
        let get_static_method_id = (**env).GetStaticMethodID?;
        let call_static_object_method = (**env).CallStaticObjectMethod?;
        let get_method_id = (**env).GetMethodID?;
        let call_object_method = (**env).CallObjectMethod?;
        let get_string_utf_chars = (**env).GetStringUTFChars?;
        let release_string_utf_chars = (**env).ReleaseStringUTFChars?;

        let locale_class_name = std::ffi::CString::new("java/util/Locale").ok()?;
        let locale_class = find_class(env, locale_class_name.as_ptr());
        if locale_class.is_null() {
            return None;
        }

        let get_default_sig = std::ffi::CString::new("()Ljava/util/Locale;").ok()?;
        let get_default_name = std::ffi::CString::new("getDefault").ok()?;
        let get_default_mid = get_static_method_id(
            env,
            locale_class,
            get_default_name.as_ptr(),
            get_default_sig.as_ptr(),
        );
        if get_default_mid.is_null() {
            return None;
        }

        let default_locale = call_static_object_method(env, locale_class, get_default_mid);
        if default_locale.is_null() {
            return None;
        }

        let to_lang_tag_sig = std::ffi::CString::new("()Ljava/lang/String;").ok()?;
        let to_lang_tag_name = std::ffi::CString::new("toLanguageTag").ok()?;
        let to_lang_tag_mid = get_method_id(
            env,
            locale_class,
            to_lang_tag_name.as_ptr(),
            to_lang_tag_sig.as_ptr(),
        );
        if to_lang_tag_mid.is_null() {
            return None;
        }

        let jstr = call_object_method(env, default_locale, to_lang_tag_mid);
        if jstr.is_null() {
            return None;
        }

        let cstr_ptr = get_string_utf_chars(env, jstr as _, std::ptr::null_mut());
        if cstr_ptr.is_null() {
            return None;
        }

        let result = std::ffi::CStr::from_ptr(cstr_ptr)
            .to_str()
            .ok()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);

        release_string_utf_chars(env, jstr as _, cstr_ptr);

        result
    }
}

#[cfg(target_os = "android")]
fn query_android_sys_prop_locale() -> Option<String> {
    extern "C" {
        fn __system_property_get(
            name: *const std::os::raw::c_char,
            value: *mut std::os::raw::c_char,
        ) -> std::os::raw::c_int;
    }

    const PROPS: &[&[u8]] = &[
        b"persist.sys.locale\0",
        b"ro.product.locale\0",
        b"persist.sys.language\0",
    ];

    PROPS.iter().find_map(|&prop| {
        let mut buf = [0u8; 128];
        let len =
            unsafe { __system_property_get(prop.as_ptr() as *const _, buf.as_mut_ptr() as *mut _) };
        (len > 0).then(|| {
            std::str::from_utf8(&buf[..len as usize])
                .ok()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })?
    })
}

#[cfg(target_os = "android")]
pub fn detect_locale_tag() -> String {
    query_android_jni_locale()
        .or_else(query_android_sys_prop_locale)
        .unwrap_or_else(|| "en-US".to_string())
}

/// Extracts locale tag from CLI arguments (--lang <tag>, --lang=<tag>, -l <tag>).
#[allow(dead_code)]
pub fn parse_cli_locale(args: impl IntoIterator<Item = impl AsRef<str>>) -> Option<String> {
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        let s = arg.as_ref();
        if s == "--lang" || s == "-l" {
            if let Some(val) = iter.next() {
                let v = val.as_ref().trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        } else if let Some(val) = s
            .strip_prefix("--lang=")
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            return Some(val.to_string());
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn detect_macos_locale() -> Option<String> {
    extern "C" {
        fn CFLocaleCopyCurrent() -> *const std::ffi::c_void;
        fn CFLocaleGetIdentifier(loc: *const std::ffi::c_void) -> *const std::ffi::c_void;
        fn CFStringGetCString(
            str_ref: *const std::ffi::c_void,
            buf: *mut std::os::raw::c_char,
            size: isize,
            enc: u32,
        ) -> bool;
        fn CFRelease(cf: *const std::ffi::c_void);
    }
    unsafe {
        let loc = CFLocaleCopyCurrent();
        if loc.is_null() {
            return None;
        }
        let ident = CFLocaleGetIdentifier(loc);
        let mut buf = [0u8; 64];
        let ok = !ident.is_null()
            && CFStringGetCString(ident, buf.as_mut_ptr() as _, buf.len() as isize, 0x08000100);
        CFRelease(loc);
        ok.then(|| {
            std::ffi::CStr::from_bytes_until_nul(&buf)
                .ok()?
                .to_str()
                .ok()
                .map(str::to_string)
        })?
    }
}

#[cfg(target_os = "windows")]
fn detect_windows_locale() -> Option<String> {
    extern "system" {
        fn GetUserDefaultLocaleName(buf: *mut u16, len: i32) -> i32;
    }
    let mut buf = [0u16; 85];
    let len = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
    (len > 1).then(|| String::from_utf16(&buf[..(len as usize - 1)]).ok())?
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
fn detect_env_locale() -> Option<String> {
    ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"]
        .into_iter()
        .find_map(|var| {
            let val = std::env::var(var).ok()?;
            let clean = val.trim().split(['.', ':']).next().unwrap_or("");
            (!clean.is_empty() && clean != "C" && clean != "POSIX").then(|| clean.to_string())
        })
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
fn detect_os_locale() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        detect_macos_locale()
    }
    #[cfg(target_os = "windows")]
    {
        detect_windows_locale()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
pub fn detect_locale_tag() -> String {
    parse_cli_locale(std::env::args().skip(1))
        .or_else(detect_os_locale)
        .or_else(detect_env_locale)
        .unwrap_or_else(|| "en-US".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_locales_load_and_validate() {
        let locales = get_locales_list();
        assert_eq!(locales.len(), 8);

        for loc in locales {
            assert!(!loc.locale.is_empty());
            assert!(!loc.language_name.is_empty());

            // Menu
            assert!(!loc.menu.title.is_empty());
            assert!(!loc.menu.subtitle.is_empty());
            assert!(!loc.menu.play_game.is_empty());
            assert!(!loc.menu.level_select.is_empty());
            assert!(!loc.menu.sound_on.is_empty());
            assert!(!loc.menu.sound_off.is_empty());

            // Level Select
            assert!(!loc.level_select.title.is_empty());
            assert!(!loc.level_select.back.is_empty());
            assert!(!loc.level_select.size_small.is_empty());
            assert!(!loc.level_select.size_medium.is_empty());
            assert!(!loc.level_select.size_big.is_empty());
            assert!(!loc.level_select.diff_relaxed.is_empty());
            assert!(!loc.level_select.diff_challenging.is_empty());
            assert!(!loc.level_select.diff_hard.is_empty());
            assert!(!loc.level_select.generating_levels.is_empty());

            // HUD
            assert!(!loc.hud.level_template.is_empty());
            assert!(!loc.hud.moves.is_empty());
            assert!(!loc.hud.par.is_empty());
            assert!(!loc.hud.stats_template.is_empty());

            let lvl_str = loc.hud.format_level(42);
            assert!(
                lvl_str.contains("42"),
                "format_level in {} should contain 42",
                loc.locale
            );

            let stats_str = loc.hud.format_stats(5, 10);
            assert!(
                stats_str.contains('5') && stats_str.contains("10"),
                "format_stats in {} should contain 5 and 10",
                loc.locale
            );

            // Win Modal
            assert!(!loc.win_modal.title.is_empty());
            assert!(!loc.win_modal.completed_template.is_empty());
            assert!(!loc.win_modal.eval_perfect.is_empty());
            assert!(!loc.win_modal.eval_great.is_empty());
            assert!(!loc.win_modal.eval_good.is_empty());
            assert!(!loc.win_modal.next_level.is_empty());
            assert!(!loc.win_modal.replay.is_empty());
            assert!(!loc.win_modal.menu.is_empty());

            let completed = loc.win_modal.format_completed(8, 10);
            assert!(
                completed.contains('8') && completed.contains("10"),
                "format_completed in {} should contain 8 and 10",
                loc.locale
            );

            assert_eq!(
                loc.win_modal.rating_evaluation(10, 10),
                loc.win_modal.eval_perfect
            );
            assert_eq!(
                loc.win_modal.rating_evaluation(12, 10),
                loc.win_modal.eval_great
            );
            assert_eq!(
                loc.win_modal.rating_evaluation(20, 10),
                loc.win_modal.eval_good
            );

            // Tab helper tests
            for size in FieldSize::ALL {
                assert!(!loc.level_select.size_label(size).is_empty());
            }
            for diff in DifficultyTier::ALL {
                assert!(!loc.level_select.difficulty_label(diff).is_empty());
            }
        }
    }

    #[test]
    fn test_locale_normalization_and_resolution() {
        assert_eq!(resolve_locale("ru-RU").locale, "ru-RU");
        assert_eq!(resolve_locale("ru_RU").locale, "ru-RU");
        assert_eq!(resolve_locale("ru+RU").locale, "ru-RU");
        assert_eq!(resolve_locale("RU").locale, "ru-RU");
        assert_eq!(resolve_locale("ru_KZ").locale, "ru-RU");

        assert_eq!(resolve_locale("es-ES").locale, "es-ES");
        assert_eq!(resolve_locale("es_MX").locale, "es-ES");
        assert_eq!(resolve_locale("es").locale, "es-ES");

        assert_eq!(resolve_locale("de-DE").locale, "de-DE");
        assert_eq!(resolve_locale("de_AT").locale, "de-DE");
        assert_eq!(resolve_locale("de").locale, "de-DE");

        assert_eq!(resolve_locale("fr-FR").locale, "fr-FR");
        assert_eq!(resolve_locale("fr_CA").locale, "fr-FR");
        assert_eq!(resolve_locale("fr").locale, "fr-FR");

        assert_eq!(resolve_locale("ja-JP").locale, "ja-JP");
        assert_eq!(resolve_locale("ja_JP").locale, "ja-JP");
        assert_eq!(resolve_locale("ja").locale, "ja-JP");
        assert_eq!(resolve_locale("jp").locale, "ja-JP");
        assert_eq!(resolve_locale("jp-JP").locale, "ja-JP");

        assert_eq!(resolve_locale("zh-CN").locale, "zh-CN");
        assert_eq!(resolve_locale("zh_CN").locale, "zh-CN");
        assert_eq!(resolve_locale("zh").locale, "zh-CN");
        assert_eq!(resolve_locale("cn").locale, "zh-CN");
        assert_eq!(resolve_locale("zh-hans").locale, "zh-CN");

        assert_eq!(resolve_locale("ko-KR").locale, "ko-KR");
        assert_eq!(resolve_locale("ko_KR").locale, "ko-KR");
        assert_eq!(resolve_locale("ko").locale, "ko-KR");
        assert_eq!(resolve_locale("kr").locale, "ko-KR");

        assert_eq!(resolve_locale("en-US").locale, "en-US");
        assert_eq!(resolve_locale("en_GB").locale, "en-US");
        assert_eq!(resolve_locale("en").locale, "en-US");

        // Unknown locale falls back to en-US
        assert_eq!(resolve_locale("it-IT").locale, "en-US");
        assert_eq!(resolve_locale("unknown").locale, "en-US");
    }

    #[test]
    fn test_parse_cli_locale() {
        assert_eq!(
            parse_cli_locale(["--lang", "ru-RU"]),
            Some("ru-RU".to_string())
        );
        assert_eq!(parse_cli_locale(["-l", "es-ES"]), Some("es-ES".to_string()));
        assert_eq!(
            parse_cli_locale(["--lang=de-DE"]),
            Some("de-DE".to_string())
        );
        assert_eq!(parse_cli_locale(["--lang=fr"]), Some("fr".to_string()));
        assert_eq!(
            parse_cli_locale(["--other", "val", "-l", "pt+BR"]),
            Some("pt+BR".to_string())
        );
        assert_eq!(parse_cli_locale(["--other", "val"]), None);
    }

    #[test]
    fn test_detect_locale_tag() {
        let tag = detect_locale_tag();
        assert!(!tag.is_empty(), "Detected locale tag should not be empty");
        let resolved = resolve_locale(&tag);
        assert!(!resolved.locale.is_empty());
    }

    #[test]
    fn test_font_covers_all_locales() {
        let font_bytes = include_bytes!("../../assets/fonts/game_font.ttf");
        assert!(font_bytes.len() > 12);

        let num_tables = u16::from_be_bytes([font_bytes[4], font_bytes[5]]) as usize;
        let mut cmap_offset = None;
        for i in 0..num_tables {
            let offset = 12 + i * 16;
            let tag = &font_bytes[offset..offset + 4];
            if tag == b"cmap" {
                let off = u32::from_be_bytes([
                    font_bytes[offset + 8],
                    font_bytes[offset + 9],
                    font_bytes[offset + 10],
                    font_bytes[offset + 11],
                ]) as usize;
                cmap_offset = Some(off);
                break;
            }
        }
        let cmap_offset = cmap_offset.expect("game_font.ttf must have cmap table");
        let num_subtables =
            u16::from_be_bytes([font_bytes[cmap_offset + 2], font_bytes[cmap_offset + 3]]) as usize;

        let mut supported_chars = std::collections::HashSet::new();
        for i in 0..num_subtables {
            let sub_rec = cmap_offset + 4 + i * 8;
            let sub_off = cmap_offset
                + u32::from_be_bytes([
                    font_bytes[sub_rec + 4],
                    font_bytes[sub_rec + 5],
                    font_bytes[sub_rec + 6],
                    font_bytes[sub_rec + 7],
                ]) as usize;
            let fmt = u16::from_be_bytes([font_bytes[sub_off], font_bytes[sub_off + 1]]);
            if fmt == 4 {
                let seg_count =
                    u16::from_be_bytes([font_bytes[sub_off + 6], font_bytes[sub_off + 7]]) as usize
                        / 2;
                let end_codes_start = sub_off + 14;
                let start_codes_start = end_codes_start + seg_count * 2 + 2;
                for s in 0..seg_count {
                    let end_code = u16::from_be_bytes([
                        font_bytes[end_codes_start + s * 2],
                        font_bytes[end_codes_start + s * 2 + 1],
                    ]);
                    let start_code = u16::from_be_bytes([
                        font_bytes[start_codes_start + s * 2],
                        font_bytes[start_codes_start + s * 2 + 1],
                    ]);
                    if start_code == 0xFFFF {
                        break;
                    }
                    for c in start_code..=end_code {
                        if let Some(ch) = char::from_u32(c as u32) {
                            supported_chars.insert(ch);
                        }
                    }
                }
            }
        }

        for loc in get_locales_list() {
            let check_str = |field_name: &str, s: &str| {
                for c in s.chars() {
                    if c != ' ' && c != '\n' && c != '\t' {
                        assert!(
                            supported_chars.contains(&c),
                            "Character '{}' (U+{:04X}) in {} [{}] missing from game_font.ttf",
                            c,
                            c as u32,
                            loc.locale,
                            field_name
                        );
                    }
                }
            };
            check_str("menu.title", &loc.menu.title);
            check_str("menu.subtitle", &loc.menu.subtitle);
            check_str("menu.play_game", &loc.menu.play_game);
            check_str("menu.level_select", &loc.menu.level_select);
            check_str("menu.sound_on", &loc.menu.sound_on);
            check_str("menu.sound_off", &loc.menu.sound_off);
            check_str("level_select.title", &loc.level_select.title);
            check_str("level_select.back", &loc.level_select.back);
            check_str("level_select.size_small", &loc.level_select.size_small);
            check_str("level_select.size_medium", &loc.level_select.size_medium);
            check_str("level_select.size_big", &loc.level_select.size_big);
            check_str("level_select.diff_relaxed", &loc.level_select.diff_relaxed);
            check_str(
                "level_select.diff_challenging",
                &loc.level_select.diff_challenging,
            );
            check_str("level_select.diff_hard", &loc.level_select.diff_hard);
            check_str(
                "level_select.generating_levels",
                &loc.level_select.generating_levels,
            );
            check_str("hud.level_template", &loc.hud.level_template);
            check_str("hud.moves", &loc.hud.moves);
            check_str("hud.par", &loc.hud.par);
            check_str("hud.stats_template", &loc.hud.stats_template);
            check_str("win_modal.title", &loc.win_modal.title);
            check_str(
                "win_modal.completed_template",
                &loc.win_modal.completed_template,
            );
            check_str("win_modal.eval_perfect", &loc.win_modal.eval_perfect);
            check_str("win_modal.eval_great", &loc.win_modal.eval_great);
            check_str("win_modal.eval_good", &loc.win_modal.eval_good);
            check_str("win_modal.next_level", &loc.win_modal.next_level);
            check_str("win_modal.replay", &loc.win_modal.replay);
            check_str("win_modal.menu", &loc.win_modal.menu);
        }
    }
}
