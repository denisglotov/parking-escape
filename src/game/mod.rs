pub mod board;
pub mod i18n;
pub mod level;
pub mod obstacle;
pub mod solver;
pub mod theme;
pub mod vehicle;

#[allow(unused_imports)]
pub use board::Board;
#[allow(unused_imports)]
pub use i18n::{detect_locale_tag, resolve_locale, LocaleStrings};
#[allow(unused_imports)]
pub use obstacle::{Obstacle, ObstacleKind};
#[allow(unused_imports)]
pub use theme::Theme;
