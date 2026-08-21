#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundTrigger {
    Slide,
    Bump,
    Win,
    ButtonClick,
    ExitDrive,
}

#[cfg(target_arch = "wasm32")]
mod wasm_backend {
    use super::SoundTrigger;

    #[link(wasm_import_module = "env")]
    extern "C" {
        fn play_sound_slide();
        fn play_sound_bump();
        fn play_sound_win();
        fn play_sound_click();
        fn play_sound_exit();
    }

    pub struct SoundBackend;

    impl SoundBackend {
        pub async fn new() -> Self {
            Self
        }

        pub fn play(&self, trigger: SoundTrigger) {
            unsafe {
                match trigger {
                    SoundTrigger::Slide => play_sound_slide(),
                    SoundTrigger::Bump => play_sound_bump(),
                    SoundTrigger::Win => play_sound_win(),
                    SoundTrigger::ButtonClick => play_sound_click(),
                    SoundTrigger::ExitDrive => play_sound_exit(),
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native_backend {
    use super::SoundTrigger;
    use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

    pub struct SoundBackend {
        snd_slide: Option<Sound>,
        snd_bump: Option<Sound>,
        snd_win: Option<Sound>,
        snd_click: Option<Sound>,
        snd_exit: Option<Sound>,
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
                snd_win: load_sound_from_bytes(include_bytes!("../assets/audio/win.wav"))
                    .await
                    .ok(),
                snd_click: load_sound_from_bytes(include_bytes!("../assets/audio/click.wav"))
                    .await
                    .ok(),
                snd_exit: load_sound_from_bytes(include_bytes!("../assets/audio/exit_drive.wav"))
                    .await
                    .ok(),
            }
        }

        pub fn play(&self, trigger: SoundTrigger) {
            let sound = match trigger {
                SoundTrigger::Slide => &self.snd_slide,
                SoundTrigger::Bump => &self.snd_bump,
                SoundTrigger::Win => &self.snd_win,
                SoundTrigger::ButtonClick => &self.snd_click,
                SoundTrigger::ExitDrive => &self.snd_exit,
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

    pub fn play(&self, trigger: SoundTrigger) {
        if self.enabled {
            self.backend.play(trigger);
        }
    }

    pub fn toggle_sound(&mut self) {
        self.enabled = !self.enabled;
    }
}
