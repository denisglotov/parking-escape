mod audio;
mod game;
mod ui;

use audio::{SoundManager, SoundTrigger};
use game::level::{LevelRecord, LevelRepository, PackType};
use game::Board;
use macroquad::prelude::*;
use std::collections::HashMap;
use ui::hud::{render_hud, HudAction};
use ui::level_select::{render_level_select, LevelSelectAction};
use ui::menu::{render_main_menu, MenuAction};
use ui::renderer::render_board;
use ui::win_modal::{render_win_modal, WinModalAction};
use ui::{BoardLayout, TextureStore, WaterRippleManager, THEME};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppScene {
    MainMenu,
    LevelSelect,
    Playing,
    LevelComplete,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Parking Escape".to_string(),
        window_width: 540,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 1. Initialize Audio and Load Assets
    let mut sound = SoundManager::new().await;
    let textures = TextureStore::load_all().await;
    let repo = LevelRepository::load_embedded().expect("Failed to load embedded level packs");
    let mut water_ripples = WaterRippleManager::new();

    let mut records: HashMap<(PackType, usize), LevelRecord> = HashMap::new();
    let mut current_pack = PackType::Grid6x6;
    let mut current_level_idx = 0;
    let mut current_board: Board = repo
        .get_level(current_pack, current_level_idx)
        .expect("Initial level missing")
        .to_board();

    let mut scene = AppScene::MainMenu;

    loop {
        clear_background(THEME.bg_dark);
        let screen_w = screen_width();
        let screen_h = screen_height();
        let dt = get_frame_time();

        match scene {
            AppScene::MainMenu => {
                match render_main_menu(sound.enabled, &textures, screen_w, screen_h) {
                    MenuAction::Play => {
                        sound.play(SoundTrigger::ButtonClick);
                        current_board = repo
                            .get_level(current_pack, current_level_idx)
                            .unwrap()
                            .to_board();
                        water_ripples.clear();
                        scene = AppScene::Playing;
                    }
                    MenuAction::SelectLevels => {
                        sound.play(SoundTrigger::ButtonClick);
                        scene = AppScene::LevelSelect;
                    }
                    MenuAction::ToggleSound => {
                        sound.toggle_sound();
                    }
                    MenuAction::None => {}
                }
            }

            AppScene::LevelSelect => {
                match render_level_select(
                    &repo,
                    &records,
                    &mut current_pack,
                    &textures,
                    screen_w,
                    screen_h,
                ) {
                    LevelSelectAction::SelectLevel(pack, idx) => {
                        sound.play(SoundTrigger::ButtonClick);
                        current_pack = pack;
                        current_level_idx = idx;
                        current_board = repo
                            .get_level(current_pack, current_level_idx)
                            .unwrap()
                            .to_board();
                        water_ripples.clear();
                        scene = AppScene::Playing;
                    }
                    LevelSelectAction::BackToMenu => {
                        sound.play(SoundTrigger::ButtonClick);
                        scene = AppScene::MainMenu;
                    }
                    LevelSelectAction::None => {}
                }
            }

            AppScene::Playing | AppScene::LevelComplete => {
                let current_level = repo
                    .get_level(current_pack, current_level_idx)
                    .expect("Active level not found");

                let layout = BoardLayout::calculate(
                    screen_w,
                    screen_h,
                    current_board.width,
                    current_board.height,
                );

                // Touch & Drag interaction
                let mouse_pos = mouse_position();
                let mouse_y_in_board = mouse_pos.1 > layout.hud_height;

                if scene == AppScene::Playing {
                    if is_mouse_button_pressed(MouseButton::Left) && mouse_y_in_board {
                        if current_board.theme == game::Theme::Marine {
                            water_ripples.spawn_touch_ripple(
                                mouse_pos.0,
                                mouse_pos.1,
                                layout.cell_size,
                            );
                        }
                        current_board.handle_touch_down(
                            mouse_pos.0,
                            mouse_pos.1,
                            layout.origin_x,
                            layout.origin_y,
                            layout.cell_size,
                        );
                    } else if is_mouse_button_down(MouseButton::Left) {
                        if let Some(trigger) = current_board.handle_touch_move(
                            mouse_pos.0,
                            mouse_pos.1,
                            layout.cell_size,
                        ) {
                            if current_board.theme == game::Theme::Marine {
                                water_ripples.spawn_impact_ripple(
                                    mouse_pos.0,
                                    mouse_pos.1,
                                    layout.cell_size,
                                    1.0,
                                );
                            }
                            sound.play(SoundTrigger::Bump);
                            sound.play(trigger);
                        }
                    } else if is_mouse_button_released(MouseButton::Left) {
                        if let Some(trigger) = current_board.handle_touch_up() {
                            if trigger == SoundTrigger::Alarm || trigger == SoundTrigger::Siren {
                                sound.play(SoundTrigger::Bump);
                            }
                            sound.play(trigger);
                            if trigger == SoundTrigger::Win {
                                sound.play(SoundTrigger::ExitDrive);
                            }
                        }
                    }

                    // Update board physics, vehicle inertia coasting, and animations
                    if let Some(trigger) = current_board.update(dt) {
                        if trigger == SoundTrigger::Alarm || trigger == SoundTrigger::Siren {
                            sound.play(SoundTrigger::Bump);
                        }
                        sound.play(trigger);
                        if trigger == SoundTrigger::Win {
                            sound.play(SoundTrigger::ExitDrive);
                        }
                    }

                    // Update water ripples
                    water_ripples.update(dt);

                    // Check win transition to victory modal
                    if current_board.is_won && current_board.exit_animation_progress >= 0.85 {
                        // Record completion stats
                        records
                            .entry((current_pack, current_level_idx))
                            .or_default()
                            .record_win(
                                current_board.move_count,
                                current_level.calculate_stars(current_board.move_count),
                            );

                        scene = AppScene::LevelComplete;
                    }
                }

                // Render Parking Lot and Vehicles
                render_board(&current_board, &layout, &textures, &water_ripples);

                // Render Top HUD
                match render_hud(
                    current_level,
                    current_board.move_count,
                    !current_board.history.is_empty(),
                    sound.enabled,
                    &textures,
                    screen_w,
                ) {
                    HudAction::BackToMenu => {
                        sound.play(SoundTrigger::ButtonClick);
                        scene = AppScene::LevelSelect;
                    }
                    HudAction::Undo => {
                        if current_board.undo() {
                            sound.play(SoundTrigger::Slide);
                        }
                    }
                    HudAction::Reset => {
                        sound.play(SoundTrigger::ButtonClick);
                        current_board.reset();
                        water_ripples.clear();
                    }
                    HudAction::ToggleSound => {
                        sound.toggle_sound();
                    }
                    HudAction::None => {}
                }

                // Render Win Modal if game is completed
                if scene == AppScene::LevelComplete {
                    let total_levels = repo.get_pack(current_pack).len();
                    let has_next = current_level_idx + 1 < total_levels;

                    match render_win_modal(
                        current_level,
                        current_board.move_count,
                        has_next,
                        &textures,
                        screen_w,
                        screen_h,
                    ) {
                        WinModalAction::NextLevel => {
                            sound.play(SoundTrigger::ButtonClick);
                            current_level_idx += 1;
                            current_board = repo
                                .get_level(current_pack, current_level_idx)
                                .unwrap()
                                .to_board();
                            water_ripples.clear();
                            scene = AppScene::Playing;
                        }
                        WinModalAction::Replay => {
                            sound.play(SoundTrigger::ButtonClick);
                            current_board.reset();
                            water_ripples.clear();
                            scene = AppScene::Playing;
                        }
                        WinModalAction::LevelSelect => {
                            sound.play(SoundTrigger::ButtonClick);
                            scene = AppScene::LevelSelect;
                        }
                        WinModalAction::None => {}
                    }
                }
            }
        }

        next_frame().await;
    }
}
