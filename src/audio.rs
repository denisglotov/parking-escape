use crate::game::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundTrigger {
    Slide,
    Bump,
    Alarm,
    Siren,
    Win,
    ButtonClick,
    ExitDrive,
}

#[cfg(target_arch = "wasm32")]
mod wasm_backend {
    use super::SoundTrigger;
    use crate::game::Theme;

    #[link(wasm_import_module = "env")]
    extern "C" {
        fn play_sound_slide(theme_code: i32);
        fn play_sound_bump(theme_code: i32);
        fn play_sound_alarm(theme_code: i32);
        fn play_sound_siren(theme_code: i32);
        fn play_sound_win();
        fn play_sound_click();
        fn play_sound_exit(theme_code: i32);
    }

    pub struct SoundBackend;

    impl SoundBackend {
        pub async fn new() -> Self {
            Self
        }

        pub fn play(&self, trigger: SoundTrigger, theme: Theme) {
            let theme_code = match theme {
                Theme::City => 0,
                Theme::Marine => 1,
                Theme::Railroad => 2,
            };
            unsafe {
                match trigger {
                    SoundTrigger::Slide => play_sound_slide(theme_code),
                    SoundTrigger::Bump => play_sound_bump(theme_code),
                    SoundTrigger::Alarm => play_sound_alarm(theme_code),
                    SoundTrigger::Siren => play_sound_siren(theme_code),
                    SoundTrigger::Win => play_sound_win(),
                    SoundTrigger::ButtonClick => play_sound_click(),
                    SoundTrigger::ExitDrive => play_sound_exit(theme_code),
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native_backend {
    use super::SoundTrigger;
    use crate::game::Theme;
    use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

    pub struct SoundBackend {
        snd_slide: Option<Sound>,
        snd_bump: Option<Sound>,
        snd_alarm: Option<Sound>,
        snd_siren: Option<Sound>,
        snd_win: Option<Sound>,
        snd_click: Option<Sound>,
        snd_exit: Option<Sound>,

        snd_marine_slide: Option<Sound>,
        snd_marine_bump: Option<Sound>,
        snd_marine_alarm: Option<Sound>,
        snd_marine_siren: Option<Sound>,
        snd_marine_exit: Option<Sound>,

        snd_rail_slide: Option<Sound>,
        snd_rail_bump: Option<Sound>,
        snd_rail_alarm: Option<Sound>,
        snd_rail_siren: Option<Sound>,
        snd_rail_exit: Option<Sound>,
    }

    impl SoundBackend {
        pub async fn new() -> Self {
            Self {
                snd_slide: load_sound_from_bytes(include_bytes!("../assets/audio/slide.wav"))
                    .await
                    .ok(),
                snd_bump: load_sound_from_bytes(include_bytes!("../assets/audio/bump.wav"))
                    .await
                    .ok(),
                snd_alarm: load_sound_from_bytes(include_bytes!("../assets/audio/alarm.wav"))
                    .await
                    .ok(),
                snd_siren: load_sound_from_bytes(include_bytes!("../assets/audio/siren.wav"))
                    .await
                    .ok(),
                snd_win: load_sound_from_bytes(include_bytes!("../assets/audio/win.wav"))
                    .await
                    .ok(),
                snd_click: load_sound_from_bytes(include_bytes!("../assets/audio/click.wav"))
                    .await
                    .ok(),
                snd_exit: load_sound_from_bytes(include_bytes!("../assets/audio/exit_drive.wav"))
                    .await
                    .ok(),

                snd_marine_slide: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/marine_slide.wav"
                ))
                .await
                .ok(),
                snd_marine_bump: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/marine_bump.wav"
                ))
                .await
                .ok(),
                snd_marine_alarm: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/marine_alarm.wav"
                ))
                .await
                .ok(),
                snd_marine_siren: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/marine_siren.wav"
                ))
                .await
                .ok(),
                snd_marine_exit: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/marine_exit.wav"
                ))
                .await
                .ok(),

                snd_rail_slide: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/rail_slide.wav"
                ))
                .await
                .ok(),
                snd_rail_bump: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/rail_bump.wav"
                ))
                .await
                .ok(),
                snd_rail_alarm: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/rail_alarm.wav"
                ))
                .await
                .ok(),
                snd_rail_siren: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/rail_siren.wav"
                ))
                .await
                .ok(),
                snd_rail_exit: load_sound_from_bytes(include_bytes!(
                    "../assets/audio/rail_exit.wav"
                ))
                .await
                .ok(),
            }
        }

        pub fn play(&self, trigger: SoundTrigger, theme: Theme) {
            let sound = match (trigger, theme) {
                (SoundTrigger::Slide, Theme::Marine) => &self.snd_marine_slide,
                (SoundTrigger::Slide, Theme::City) => &self.snd_slide,
                (SoundTrigger::Slide, Theme::Railroad) => &self.snd_rail_slide,

                (SoundTrigger::Bump, Theme::Marine) => &self.snd_marine_bump,
                (SoundTrigger::Bump, Theme::City) => &self.snd_bump,
                (SoundTrigger::Bump, Theme::Railroad) => &self.snd_rail_bump,

                (SoundTrigger::Alarm, Theme::Marine) => &self.snd_marine_alarm,
                (SoundTrigger::Alarm, Theme::City) => &self.snd_alarm,
                (SoundTrigger::Alarm, Theme::Railroad) => &self.snd_rail_alarm,

                (SoundTrigger::Siren, Theme::Marine) => &self.snd_marine_siren,
                (SoundTrigger::Siren, Theme::City) => &self.snd_siren,
                (SoundTrigger::Siren, Theme::Railroad) => &self.snd_rail_siren,

                (SoundTrigger::ExitDrive, Theme::Marine) => &self.snd_marine_exit,
                (SoundTrigger::ExitDrive, Theme::City) => &self.snd_exit,
                (SoundTrigger::ExitDrive, Theme::Railroad) => &self.snd_rail_exit,

                (SoundTrigger::Win, _) => &self.snd_win,
                (SoundTrigger::ButtonClick, _) => &self.snd_click,
            };

            if let Some(snd) = sound {
                play_sound_once(snd);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_backend::SoundBackend;

#[cfg(not(target_arch = "wasm32"))]
pub use native_backend::SoundBackend;

pub struct SoundManager {
    backend: SoundBackend,
    pub enabled: bool,
}

impl SoundManager {
    pub async fn new() -> Self {
        Self {
            backend: SoundBackend::new().await,
            enabled: true,
        }
    }

    pub fn play(&self, trigger: SoundTrigger, theme: Theme) {
        if self.enabled {
            self.backend.play(trigger, theme);
        }
    }

    pub fn toggle_sound(&mut self) {
        self.enabled = !self.enabled;
    }
}
