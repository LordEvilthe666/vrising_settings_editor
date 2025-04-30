/* V Rising Server Settings Editor
 * Copyright (C) 2025 LordEvilthe666
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]

extern crate rust_i18n;
extern crate sys_locale;

use crate::settings::{
    AchievementId, CastleDamageMode, CastleHeartDamageMode, CastleHeartLimitType,
    DeathContainerPermission, GameDifficulty, GameModeType, PlayerDamageMode, PvPProtectionMode,
    RelicSpawnType, ResearchId, ServerGameSettings, StarterEquipmentId, StarterResourcesId,
    TimeZone, VBloodUnitId, WarEventDuration, WarEventInterval,
};
use eframe::egui;
use std::path::PathBuf;
use strum::IntoEnumIterator;

mod settings;

i18n!("locales", fallback = "en");

#[derive(Clone, Copy, Debug, PartialEq)]
enum PendingAction {
    NewFile,
    OpenFile,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum AppTab {
    General,
    PvpCastle,
    Modifiers,
    Lists,
}

struct VrisingEditorApp {
    settings: Option<ServerGameSettings>,
    current_filepath: Option<PathBuf>,
    error_message: Option<String>,
    dirty: bool,
    new_vblood_id_to_add: VBloodUnitId,
    new_achievement_to_add: AchievementId,
    new_research_to_add: ResearchId,
    pending_action: Option<PendingAction>,
    current_tab: AppTab,
    current_language: String,
}

impl VrisingEditorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let default_lang = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        let available_locales = rust_i18n::available_locales!();
        let system_locale_base = default_lang.split('-').next().unwrap_or("en");

        let current_language = if available_locales.contains(&system_locale_base) {
            system_locale_base.to_string()
        } else if available_locales.contains(&default_lang.as_str()) {
             default_lang
        }
         else {
            "en".to_string()
        };

        rust_i18n::set_locale(&current_language);

        Self {
            settings: None,
            current_filepath: None,
            error_message: None,
            dirty: false,
            new_vblood_id_to_add: VBloodUnitId::default(),
            new_achievement_to_add: AchievementId::default(),
            new_research_to_add: ResearchId::default(),
            pending_action: None,
            current_tab: AppTab::General,
            current_language,
        }
    }

    fn perform_new_file(&mut self) {
        self.settings = Some(ServerGameSettings {
            name: t!("default_server_name"),
            description: t!("default_server_description"),
            settings: settings::Settings::default(),
        });
        self.current_filepath = None;
        self.error_message = None;
        self.dirty = true;
        self.pending_action = None;
        self.current_tab = AppTab::General;
    }

    fn perform_open_file(&mut self) {
        self.error_message = None;
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(&t!("json_file_filter_name"), &["json"])
            .set_title(&t!("open_file_dialog_title"))
            .pick_file()
        {
            match std::fs::read_to_string(&path) {
                Ok(json_content) => {
                    match serde_json::from_str::<ServerGameSettings>(&json_content) {
                        Ok(loaded_settings) => {
                            self.settings = Some(loaded_settings);
                            self.current_filepath = Some(path);
                            self.dirty = false;
                            self.current_tab = AppTab::General;
                        }
                        Err(e) => {
                            self.settings = None;
                            self.current_filepath = None;
                            self.error_message = Some(t!("error_parsing_json", error = e.to_string()));
                            self.dirty = false;
                        }
                    }
                }
                Err(e) => {
                    self.settings = None;
                    self.current_filepath = None;
                    self.error_message = Some(t!("error_reading_file", error = e.to_string()));
                    self.dirty = false;
                }
            }
        }
        self.pending_action = None;
    }

    fn trigger_new_file(&mut self) {
        if self.dirty && self.settings.is_some() {
            self.pending_action = Some(PendingAction::NewFile);
        } else {
            self.perform_new_file();
        }
    }

    fn trigger_open_file(&mut self) {
        if self.dirty && self.settings.is_some() {
            self.pending_action = Some(PendingAction::OpenFile);
        } else {
            self.perform_open_file();
        }
    }

    fn trigger_exit(&mut self, ctx: &egui::Context) {
        if self.dirty && self.settings.is_some() {
            self.pending_action = Some(PendingAction::Exit);
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn save_file(&mut self) -> bool {
        self.error_message = None;
        if let Some(settings_to_save) = &self.settings {
            let path_to_save = if self.current_filepath.is_some() {
                self.current_filepath.clone()
            } else {
                rfd::FileDialog::new()
                    .add_filter(&t!("json_file_filter_name"), &["json"])
                    .set_file_name("ServerGameSettings.json")
                    .set_title(&t!("save_file_dialog_title"))
                    .save_file()
            };

            if let Some(path) = path_to_save {
                match serde_json::to_string_pretty(settings_to_save) {
                    Ok(json_content) => {
                        match std::fs::write(&path, json_content) {
                            Ok(_) => {
                                self.current_filepath = Some(path);
                                self.dirty = false;
                                self.error_message = Some(t!("status_save_success"));
                                return true;
                            }
                            Err(e) => {
                                self.error_message = Some(t!("error_writing_file", error = e.to_string()));
                                return false;
                            }
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(t!("error_serializing_json", error = e.to_string()));
                        return false;
                    }
                }
            } else {
                return false;
            }
        } else {
            self.error_message = Some(t!("error_no_data_to_save"));
            return false;
        }
    }

    fn save_file_as(&mut self) -> bool {
        let current_path_backup = self.current_filepath.clone();
        self.current_filepath = None;
        let saved = self.save_file();
        if !saved && self.current_filepath.is_none() {
            self.current_filepath = current_path_backup;
        }
        saved
    }

    fn mark_dirty(&mut self) {
        self.dirty = true;
        if let Some(msg) = &self.error_message {
            if msg == &t!("status_save_success") {
                self.error_message = None;
            }
        }
    }

    fn show_confirmation_dialog(&mut self, ctx: &egui::Context) {
        if let Some(action) = self.pending_action {
            let action_key = match action {
                PendingAction::NewFile => "unsaved_changes_action_new",
                PendingAction::OpenFile => "unsaved_changes_action_open",
                PendingAction::Exit => "unsaved_changes_action_exit",
            };
            let action_text = t!(action_key);
            let window_title = t!("unsaved_changes_dialog_title");
            let prompt_text = t!("unsaved_changes_prompt", action = action_text);
            let mut close_dialog = false;

            egui::Window::new(window_title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(prompt_text);
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button(t!("save_dialog_button")).clicked() {
                            if self.save_file() {
                                match action {
                                    PendingAction::NewFile => self.perform_new_file(),
                                    PendingAction::OpenFile => self.perform_open_file(),
                                    PendingAction::Exit => {
                                        ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                                    }
                                }
                                close_dialog = true;
                            }
                        }
                        if ui.button(t!("dont_save_dialog_button")).clicked() {
                            self.dirty = false;
                            match action {
                                PendingAction::NewFile => self.perform_new_file(),
                                PendingAction::OpenFile => self.perform_open_file(),
                                PendingAction::Exit => {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                                }
                            }
                            close_dialog = true;
                        }
                        if ui.button(t!("cancel_dialog_button")).clicked() {
                            close_dialog = true;
                        }
                    });
                });

            if close_dialog {
                self.pending_action = None;
            }
        }
    }

    fn set_language(&mut self, lang: &str) {
        rust_i18n::set_locale(lang);
        self.current_language = lang.to_string();
    }
}

impl eframe::App for VrisingEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        rust_i18n::set_locale(&self.current_language);

        let window_title = t!("app_title");
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(window_title));

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.dirty && self.settings.is_some() && self.pending_action.is_none() {
                self.trigger_exit(ctx);
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(t!("file_menu"), |ui| {
                    if ui.button(t!("new_button")).clicked() {
                        self.trigger_new_file();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button(t!("open_button")).clicked() {
                        self.trigger_open_file();
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(
                            self.settings.is_some() && self.dirty,
                            egui::Button::new(t!("save_button")),
                        )
                        .clicked()
                    {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(
                            self.settings.is_some(),
                            egui::Button::new(t!("save_as_button")),
                        )
                        .clicked()
                    {
                        self.save_file_as();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button(t!("exit_button")).clicked() {
                        self.trigger_exit(ctx);
                        ui.close_menu();
                    }
                });

                ui.menu_button(t!("language_menu"), |ui| {
                    let available_locales = rust_i18n::available_locales!();
                    let mut sorted_locales = available_locales.to_vec();
                    sorted_locales.sort();
                    let original_locale = self.current_language.clone();
                    for locale_code in sorted_locales {
                        rust_i18n::set_locale(locale_code);
                        let display_name = t!("language_name", fallback = locale_code);
                        if ui.selectable_label(original_locale == locale_code, display_name).clicked() {
                            self.set_language(locale_code);
                            ui.close_menu();
                            break;
                        }
                    }
                    rust_i18n::set_locale(&original_locale);
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let file_status = if let Some(path) = &self.current_filepath {
                    format!("{prefix} {}", path.display(), prefix = t!("status_opened_file_prefix"))
                } else if self.settings.is_some() {
                    t!("status_new_file")
                } else {
                    t!("status_file_not_open")
                };
                let file_status_display = format!(
                    "{}{}",
                    file_status,
                    if self.dirty { t!("status_unsaved_changes_indicator") } else { "".to_string() }
                );

                let status_text = self
                    .error_message
                    .as_deref()
                    .unwrap_or(&file_status_display);

                let label_widget = egui::Label::new(status_text).wrap(false);
                let label_response = ui.add(label_widget.sense(egui::Sense::hover()));

                if let Some(path) = &self.current_filepath {
                    label_response.on_hover_text(path.display().to_string());
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let main_ui_enabled = self.pending_action.is_none();
            ui.set_enabled(main_ui_enabled);

            if let Some(settings_data) = &mut self.settings {
                let mut changed = false;
                let is_pvp = settings_data.settings.game_mode_type == GameModeType::PvP;

                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.current_tab, AppTab::General, t!("tab_general"));
                    ui.selectable_value(&mut self.current_tab, AppTab::PvpCastle, t!("tab_pvp_castle"));
                    ui.selectable_value(&mut self.current_tab, AppTab::Modifiers, t!("tab_modifiers"));
                    ui.selectable_value(&mut self.current_tab, AppTab::Lists, t!("tab_lists"));
                });
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.current_tab {
                        AppTab::General => {
                            ui.heading(t!("general_settings_header"));
                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.label(t!("server_name_label"));
                                changed |= ui.text_edit_singleline(&mut settings_data.name).changed();
                            });
                            ui.label(t!("server_description_label"));
                            changed |= ui.text_edit_multiline(&mut settings_data.description).changed();
                            ui.separator();

                            egui::Grid::new("general_settings_grid")
                                .num_columns(2)
                                .spacing([40.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label(t!("difficulty_label")).on_hover_text(t!("difficulty_tooltip"));
                                    egui::ComboBox::from_id_source("game_difficulty_combo")
                                        .selected_text(t!(&format!("enums.game_difficulty.{}", settings_data.settings.game_difficulty)))
                                        .show_ui(ui, |ui| {
                                            for variant in GameDifficulty::iter() {
                                                changed |= ui.selectable_value(&mut settings_data.settings.game_difficulty, variant, t!(&format!("enums.game_difficulty.{}", variant))).changed();
                                            }
                                        });
                                    ui.end_row();

                                    ui.label(t!("game_mode_label")).on_hover_text(t!("game_mode_tooltip"));
                                    egui::ComboBox::from_id_source("game_mode_combo")
                                        .selected_text(t!(&format!("enums.game_mode_type.{}", settings_data.settings.game_mode_type)))
                                        .show_ui(ui, |ui| {
                                            for variant in GameModeType::iter() {
                                                changed |= ui.selectable_value(&mut settings_data.settings.game_mode_type, variant, t!(&format!("enums.game_mode_type.{}", variant))).changed();
                                            }
                                        });
                                    ui.end_row();

                                    ui.label(t!("relic_spawn_label")).on_hover_text(t!("relic_spawn_tooltip"));
                                    egui::ComboBox::from_id_source("relic_spawn_combo")
                                        .selected_text(t!(&format!("enums.relic_spawn_type.{}", settings_data.settings.relic_spawn_type)))
                                        .show_ui(ui, |ui| {
                                            for variant in RelicSpawnType::iter() {
                                                changed |= ui.selectable_value(&mut settings_data.settings.relic_spawn_type, variant, t!(&format!("enums.relic_spawn_type.{}", variant))).changed();
                                            }
                                        });
                                    ui.end_row();

                                    ui.label(t!("death_container_label")).on_hover_text(t!("death_container_tooltip"));
                                    egui::ComboBox::from_id_source("death_container_perm_combo")
                                         .selected_text(t!(&format!("enums.death_container_permission.{}", settings_data.settings.death_container_permission)))
                                        .show_ui(ui, |ui| {
                                            for variant in DeathContainerPermission::iter() {
                                                changed |= ui.selectable_value(&mut settings_data.settings.death_container_permission, variant, t!(&format!("enums.death_container_permission.{}", variant))).changed();
                                            }
                                        });
                                    ui.end_row();

                                    changed |= add_grid_row_changed(ui, t!("blood_bound_label"), t!("blood_bound_tooltip"), egui::Checkbox::new(&mut settings_data.settings.blood_bound_equipment, ""));

                                    ui.label(t!("teleport_bound_label")).on_hover_text(t!("teleport_bound_tooltip"));
                                    let mut teleport_unbound = !settings_data.settings.teleport_bound_items;
                                    let resp_teleport = ui.checkbox(&mut teleport_unbound, "");
                                    if resp_teleport.changed() { settings_data.settings.teleport_bound_items = !teleport_unbound; changed = true; }
                                    resp_teleport.on_hover_text(t!("teleport_bound_checkbox_tooltip"));
                                    ui.end_row();

                                    ui.label(t!("bat_bound_label")).on_hover_text(t!("bat_bound_tooltip"));
                                    let mut bat_unbound = !settings_data.settings.bat_bound_items;
                                    let resp_bat = ui.checkbox(&mut bat_unbound, "");
                                    if resp_bat.changed() { settings_data.settings.bat_bound_items = !bat_unbound; changed = true; }
                                    resp_bat.on_hover_text(t!("bat_bound_checkbox_tooltip"));
                                    ui.end_row();

                                    changed |= add_grid_row_changed(ui, t!("allow_chat_label"), t!("allow_chat_tooltip"), egui::Checkbox::new(&mut settings_data.settings.allow_global_chat, ""));
                                    changed |= add_grid_row_changed(ui, t!("all_waypoints_label"), t!("all_waypoints_tooltip"), egui::Checkbox::new(&mut settings_data.settings.all_waypoints_unlocked, ""));
                                    changed |= add_grid_row_changed(ui, t!("inactivity_kill_label"), t!("inactivity_kill_tooltip"), egui::Checkbox::new(&mut settings_data.settings.inactivity_kill_enabled, ""));

                                    let inactivity_enabled = settings_data.settings.inactivity_kill_enabled;
                                    ui.label(t!("inactivity_min_time_label")).on_hover_text(t!("inactivity_min_time_tooltip"));
                                    changed |= ui.add_enabled(inactivity_enabled, egui::DragValue::new(&mut settings_data.settings.inactivity_kill_time_min).speed(100.0)).changed();
                                    ui.end_row();
                                    ui.label(t!("inactivity_max_time_label")).on_hover_text(t!("inactivity_max_time_tooltip"));
                                    changed |= ui.add_enabled(inactivity_enabled, egui::DragValue::new(&mut settings_data.settings.inactivity_kill_time_max).speed(1000.0)).changed();
                                    ui.end_row();
                                    ui.label(t!("inactivity_safe_time_label")).on_hover_text(t!("inactivity_safe_time_tooltip"));
                                    changed |= ui.add_enabled(inactivity_enabled, egui::DragValue::new(&mut settings_data.settings.inactivity_kill_safe_time_addition).speed(1000.0)).changed();
                                    ui.end_row();
                                    ui.label(t!("inactivity_max_level_label")).on_hover_text(t!("inactivity_max_level_tooltip"));
                                    changed |= ui.add_enabled(inactivity_enabled, egui::DragValue::new(&mut settings_data.settings.inactivity_kill_timer_max_item_level).clamp_range(0..=255)).changed();
                                    ui.end_row();

                                    changed |= add_grid_row_changed(ui, t!("disable_dead_label"), t!("disable_dead_tooltip"), egui::Checkbox::new(&mut settings_data.settings.disable_disconnected_dead_enabled, ""));
                                    let disable_dead_enabled = settings_data.settings.disable_disconnected_dead_enabled;
                                    ui.label(t!("disable_dead_timer_label")).on_hover_text(t!("disable_dead_timer_tooltip"));
                                    changed |= ui.add_enabled(disable_dead_enabled, egui::DragValue::new(&mut settings_data.settings.disable_disconnected_dead_timer).speed(10.0)).changed();
                                    ui.end_row();

                                    changed |= add_grid_row_changed(ui, t!("sun_immunity_label"), t!("sun_immunity_tooltip"), egui::DragValue::new(&mut settings_data.settings.disconnected_sun_immunity_time).speed(10.0).clamp_range(0.0..=3600.0));
                                    changed |= add_grid_row_changed(ui, t!("clan_size_label"), t!("clan_size_tooltip"), egui::DragValue::new(&mut settings_data.settings.clan_size).clamp_range(1..=50));

                                    ui.label(t!("starter_equip_label")).on_hover_text(t!("starter_equip_tooltip"));
                                    egui::ComboBox::from_id_source("starter_equip_combo")
                                        .selected_text(t!(&format!("enums.starter_equipment_id.{}", settings_data.settings.starter_equipment_id)))
                                        .show_ui(ui, |ui| {
                                            for variant in StarterEquipmentId::iter() {
                                                changed |= ui.selectable_value(&mut settings_data.settings.starter_equipment_id, variant, t!(&format!("enums.starter_equipment_id.{}", variant))).changed();
                                            }
                                        });
                                    ui.end_row();

                                    ui.label(t!("starter_res_label")).on_hover_text(t!("starter_res_tooltip"));
                                    egui::ComboBox::from_id_source("starter_res_combo")
                                        .selected_text(t!(&format!("enums.starter_resources_id.{}", settings_data.settings.starter_resources_id)))
                                        .show_ui(ui, |ui| {
                                             for variant in StarterResourcesId::iter() {
                                                changed |= ui.selectable_value(&mut settings_data.settings.starter_resources_id, variant, t!(&format!("enums.starter_resources_id.{}", variant))).changed();
                                            }
                                        });
                                    ui.end_row();
                                    changed |= add_grid_row_changed(ui, t!("starter_prog_label"), t!("starter_prog_tooltip"), egui::DragValue::new(&mut settings_data.settings.starting_progression_level).clamp_range(0..=255));
                                });
                        }
                        AppTab::PvpCastle => {
                            ui.heading(t!("pvp_castle_settings_header"));
                            ui.separator();
                            ui.add_enabled_ui(is_pvp, |ui| {
                                ui.strong(t!("pvp_settings_subheader"));
                                ui.add_space(5.0);
                                egui::Grid::new("pvp_settings_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
                                    ui.label(t!("castle_damage_label")).on_hover_text(t!("castle_damage_tooltip"));
                                    egui::ComboBox::from_id_source("castle_damage_mode_combo")
                                        .selected_text(t!(&format!("enums.castle_damage_mode.{}", settings_data.settings.castle_damage_mode)))
                                        .show_ui(ui, |ui| { for v in CastleDamageMode::iter() { changed |= ui.selectable_value(&mut settings_data.settings.castle_damage_mode, v, t!(&format!("enums.castle_damage_mode.{}", v))).changed(); } });
                                    ui.end_row();

                                    ui.label(t!("player_damage_label")).on_hover_text(t!("player_damage_tooltip"));
                                    egui::ComboBox::from_id_source("player_damage_mode_combo")
                                        .selected_text(t!(&format!("enums.player_damage_mode.{}", settings_data.settings.player_damage_mode)))
                                        .show_ui(ui, |ui| { for v in PlayerDamageMode::iter() { changed |= ui.selectable_value(&mut settings_data.settings.player_damage_mode, v, t!(&format!("enums.player_damage_mode.{}", v))).changed(); } });
                                    ui.end_row();

                                    ui.label(t!("castle_heart_damage_label")).on_hover_text(t!("castle_heart_damage_tooltip"));
                                     egui::ComboBox::from_id_source("castle_heart_damage_mode_combo")
                                        .selected_text(t!(&format!("enums.castle_heart_damage_mode.{}", settings_data.settings.castle_heart_damage_mode)))
                                        .show_ui(ui, |ui| { for v in CastleHeartDamageMode::iter() { changed |= ui.selectable_value(&mut settings_data.settings.castle_heart_damage_mode, v, t!(&format!("enums.castle_heart_damage_mode.{}", v))).changed(); } });
                                    ui.end_row();

                                    ui.label(t!("pvp_protection_label")).on_hover_text(t!("pvp_protection_tooltip"));
                                     egui::ComboBox::from_id_source("pvp_protection_combo")
                                        .selected_text(t!(&format!("enums.pvp_protection_mode.{}", settings_data.settings.pvp_protection_mode)))
                                        .show_ui(ui, |ui| { for v in PvPProtectionMode::iter() { changed |= ui.selectable_value(&mut settings_data.settings.pvp_protection_mode, v, t!(&format!("enums.pvp_protection_mode.{}", v))).changed(); } });
                                    ui.end_row();

                                    changed |= add_grid_row_changed(ui, t!("loot_enemy_containers_label"), t!("loot_enemy_containers_tooltip"), egui::Checkbox::new(&mut settings_data.settings.can_loot_enemy_containers, ""));
                                    changed |= add_grid_row_changed(ui, t!("free_raid_label"), t!("free_raid_tooltip"), egui::Checkbox::new(&mut settings_data.settings.free_castle_raid, ""));
                                    changed |= add_grid_row_changed(ui, t!("free_claim_label"), t!("free_claim_tooltip"), egui::Checkbox::new(&mut settings_data.settings.free_castle_claim, ""));
                                    changed |= add_grid_row_changed(ui, t!("free_destroy_label"), t!("free_destroy_tooltip"), egui::Checkbox::new(&mut settings_data.settings.free_castle_destroy, ""));
                                    changed |= add_grid_row_changed(ui, t!("announce_siege_label"), t!("announce_siege_tooltip"), egui::Checkbox::new(&mut settings_data.settings.announce_siege_weapon_spawn, ""));
                                    changed |= add_grid_row_changed(ui, t!("show_siege_map_label"), t!("show_siege_map_tooltip"), egui::Checkbox::new(&mut settings_data.settings.show_siege_weapon_map_icon, ""));
                                    changed |= add_grid_row_changed(ui, t!("pvp_respawn_mod_label"), t!("pvp_respawn_mod_tooltip"), egui::DragValue::new(&mut settings_data.settings.pvp_vampire_respawn_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
                                    changed |= add_grid_row_changed(ui, t!("castle_siege_timer_label"), t!("castle_siege_timer_tooltip"), egui::DragValue::new(&mut settings_data.settings.castle_siege_timer).speed(10.0).clamp_range(60.0..=1800.0));
                                    changed |= add_grid_row_changed(ui, t!("castle_under_attack_timer_label"), t!("castle_under_attack_timer_tooltip"), egui::DragValue::new(&mut settings_data.settings.castle_under_attack_timer).speed(1.0).clamp_range(0.0..=60.0));
                                    changed |= add_grid_row_changed(ui, t!("castle_raid_timer_label"), t!("castle_raid_timer_tooltip"), egui::DragValue::new(&mut settings_data.settings.castle_raid_timer).speed(10.0).clamp_range(60.0..=3600.0));
                                    changed |= add_grid_row_changed(ui, t!("castle_raid_prot_timer_label"), t!("castle_raid_prot_timer_tooltip"), egui::DragValue::new(&mut settings_data.settings.castle_raid_protection_time).speed(100.0).clamp_range(0.0..=604800.0));
                                    changed |= add_grid_row_changed(ui, t!("castle_free_claim_timer_label"), t!("castle_free_claim_timer_tooltip"), egui::DragValue::new(&mut settings_data.settings.castle_exposed_free_claim_timer).speed(10.0).clamp_range(0.0..=3600.0));
                                    changed |= add_grid_row_changed(ui, t!("death_dura_loss_label"), t!("death_dura_loss_tooltip"), egui::DragValue::new(&mut settings_data.settings.death_durability_factor_loss).speed(0.01).clamp_range(0.0..=1.0).fixed_decimals(3));
                                    changed |= add_grid_row_changed(ui, t!("death_dura_res_label"), t!("death_dura_res_tooltip"), egui::DragValue::new(&mut settings_data.settings.death_durability_loss_factor_as_resources).speed(0.05).clamp_range(0.0..=1.0).fixed_decimals(2));
                                });
                            });

                            ui.separator();
                            ui.strong(t!("castle_settings_subheader"));
                            ui.add_space(5.0);
                            changed |= add_grid_row_changed(ui, t!("castle_relocation_label"), t!("castle_relocation_tooltip"), egui::Checkbox::new(&mut settings_data.settings.castle_relocation_enabled, ""));
                            ui.add_enabled_ui(settings_data.settings.castle_relocation_enabled, |ui|{
                                 changed |= add_grid_row_changed(ui, t!("castle_reloc_cooldown_label"), t!("castle_reloc_cooldown_tooltip"), egui::DragValue::new(&mut settings_data.settings.castle_relocation_cooldown).speed(100.0).clamp_range(0.0..=2592000.0));
                            });
                            changed |= show_castle_stat_modifiers(ui, &mut settings_data.settings.castle_stat_modifiers_global);

                            ui.separator();
                            let mut show_pis_section = settings_data.settings.player_interaction_settings.is_some();
                            if !show_pis_section {
                                let add_pis_button = ui.add_enabled(is_pvp, egui::Button::new(t!("add_pis_button_text")));
                                if add_pis_button.clicked() {
                                    settings_data.settings.player_interaction_settings = Some(Default::default());
                                    show_pis_section = true;
                                    changed = true;
                                }
                                add_pis_button.on_disabled_hover_text(t!("add_pis_button_tooltip"));
                            }

                            if show_pis_section {
                                if let Some(pis) = &mut settings_data.settings.player_interaction_settings {
                                    ui.add_enabled_ui(is_pvp, |ui|{
                                        ui.collapsing(t!("pis_header"), |ui| {
                                            changed |= show_player_interaction_settings(ui, pis);
                                        }).header_response.on_hover_text(t!("pis_header_tooltip"));
                                    });
                                }
                            }
                        }
                        AppTab::Modifiers => {
                            ui.heading(t!("modifiers_header"));
                            ui.separator();

                            ui.strong(t!("modifiers_economy_subheader"));
                            ui.add_space(5.0);
                            ui.collapsing(t!("modifiers_resources_header"), |ui| {
                                egui::Grid::new("resource_drop_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
                                    changed |= add_grid_row_changed(ui, t!("res_mod_stack_label"), t!("res_mod_stack_tooltip"), egui::DragValue::new(&mut settings_data.settings.inventory_stacks_modifier).speed(0.05).clamp_range(0.25..=3.0).fixed_decimals(2));
                                    changed |= add_grid_row_changed(ui, t!("res_mod_drop_general_label"), t!("res_mod_drop_general_tooltip"), egui::DragValue::new(&mut settings_data.settings.drop_table_modifier_general).speed(0.05).clamp_range(0.25..=3.0).fixed_decimals(2));
                                    changed |= add_grid_row_changed(ui, t!("res_mod_drop_missions_label"), t!("res_mod_drop_missions_tooltip"), egui::DragValue::new(&mut settings_data.settings.drop_table_modifier_missions).speed(0.05).clamp_range(0.25..=3.0).fixed_decimals(2));
                                    changed |= add_grid_row_changed(ui, t!("res_mod_drop_stygian_label"), t!("res_mod_drop_stygian_tooltip"), egui::DragValue::new(&mut settings_data.settings.drop_table_modifier_stygian_shards).speed(0.05).clamp_range(0.25..=3.0).fixed_decimals(2));
                                    changed |= add_grid_row_changed(ui, t!("res_mod_shard_dura_label"), t!("res_mod_shard_dura_tooltip"), egui::DragValue::new(&mut settings_data.settings.soul_shard_durability_loss_rate).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
                                    changed |= add_grid_row_changed(ui, t!("res_mod_material_yield_label"), t!("res_mod_material_yield_tooltip"), egui::DragValue::new(&mut settings_data.settings.material_yield_modifier_global).speed(0.05).clamp_range(0.25..=3.0).fixed_decimals(2));
                                    changed |= add_grid_row_changed(ui, t!("res_mod_blood_yield_label"), t!("res_mod_blood_yield_tooltip"), egui::DragValue::new(&mut settings_data.settings.blood_essence_yield_modifier).speed(0.05).clamp_range(0.25..=3.0).fixed_decimals(2));
                                });
                            });
                            ui.collapsing(t!("modifiers_crafting_header"), |ui| { changed |= show_craft_build_modifiers(ui, &mut settings_data.settings); });
                            ui.collapsing(t!("modifiers_traders_header"), |ui| { changed |= show_trader_modifiers(ui, &mut settings_data.settings.trader_modifiers); });
                            ui.separator();

                            ui.strong(t!("modifiers_env_time_subheader"));
                            ui.add_space(5.0);
                            ui.collapsing(t!("modifiers_env_damage_header"), |ui| { changed |= show_env_damage_modifiers(ui, &mut settings_data.settings); });
                            ui.collapsing(t!("modifiers_time_header"), |ui| { changed |= show_game_time_modifiers(ui, &mut settings_data.settings.game_time_modifiers); });
                            ui.separator();

                            ui.strong(t!("modifiers_stats_subheader"));
                            ui.add_space(5.0);
                            ui.collapsing(t!("modifiers_vampire_stats_header"), |ui| { changed |= show_vampire_stat_modifiers(ui, &mut settings_data.settings.vampire_stat_modifiers); });
                            ui.collapsing(t!("modifiers_unit_stats_global_header"), |ui| { changed |= show_unit_stat_modifiers(ui, "global", &mut settings_data.settings.unit_stat_modifiers_global); });
                            ui.collapsing(t!("modifiers_unit_stats_vblood_header"), |ui| { changed |= show_unit_stat_modifiers(ui, "vblood", &mut settings_data.settings.unit_stat_modifiers_v_blood); });
                            ui.collapsing(t!("modifiers_equip_stats_header"), |ui| { changed |= show_equipment_stat_modifiers(ui, &mut settings_data.settings.equipment_stat_modifiers_global); });
                            ui.collapsing(t!("modifiers_castle_common_header"), |ui| { changed |= show_castle_common_settings(ui, &mut settings_data.settings); });
                            ui.separator();

                            ui.strong(t!("modifiers_events_subheader"));
                            ui.add_space(5.0);
                            let mut show_wes_section = settings_data.settings.war_event_game_settings.is_some();
                            if !show_wes_section {
                                let add_wes_button = ui.button(t!("add_wes_button_text"));
                                if add_wes_button.clicked() {
                                    settings_data.settings.war_event_game_settings = Some(Default::default());
                                    show_wes_section = true;
                                    changed = true;
                                }
                                add_wes_button.on_hover_text(t!("add_wes_button_tooltip"));
                            }
                            if show_wes_section {
                                if let Some(wes) = &mut settings_data.settings.war_event_game_settings {
                                    ui.collapsing(t!("wes_header"), |ui| {
                                        changed |= show_war_event_settings(ui, wes);
                                    }).header_response.on_hover_text(t!("wes_header_tooltip"));
                                }
                            }
                        }
                        AppTab::Lists => {
                            ui.heading(t!("lists_header"));
                            ui.separator();
                            ui.collapsing(t!("vblood_settings_header"), |ui| { changed |= show_vblood_settings(ui, &mut settings_data.settings.v_blood_unit_settings, &mut self.new_vblood_id_to_add); });
                            ui.collapsing(t!("achievements_header"), |ui| { changed |= show_unlocked_achievements(ui, &mut settings_data.settings.unlocked_achievements, &mut self.new_achievement_to_add); });
                            ui.collapsing(t!("research_header"), |ui| { changed |= show_unlocked_research(ui, &mut settings_data.settings.unlocked_researchs, &mut self.new_research_to_add); });
                        }
                    }
                });

                if changed {
                    self.mark_dirty();
                }

            } else {
                ui.centered_and_justified(|ui| {
                    ui.label(format!("{} '{}' -> '{}' / '{}'", t!("status_file_not_open"), t!("file_menu"), t!("new_button"), t!("open_button")));
                });
            }

            if !main_ui_enabled {
                 self.show_confirmation_dialog(ctx);
            }

        });
    }
}

fn add_grid_row_changed<W: egui::Widget>(
    ui: &mut egui::Ui,
    label_text: String,
    tooltip_text: String,
    widget: W,
) -> bool {
    let label_response = ui.label(label_text).on_hover_text(tooltip_text);
    let widget_response = ui.add(widget);
    ui.end_row();
    widget_response.changed() || label_response.changed()
}

fn show_env_damage_modifiers(ui: &mut egui::Ui, settings: &mut settings::Settings) -> bool {
    let mut changed = false;
    egui::Grid::new("env_damage_modifiers_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("env_mod_blood_drain_label"), t!("env_mod_blood_drain_tooltip"), egui::DragValue::new(&mut settings.blood_drain_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("env_mod_durability_drain_label"), t!("env_mod_durability_drain_tooltip"), egui::DragValue::new(&mut settings.durability_drain_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("env_mod_garlic_strength_label"), t!("env_mod_garlic_strength_tooltip"), egui::DragValue::new(&mut settings.garlic_area_strength_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("env_mod_holy_strength_label"), t!("env_mod_holy_strength_tooltip"), egui::DragValue::new(&mut settings.holy_area_strength_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("env_mod_silver_strength_label"), t!("env_mod_silver_strength_tooltip"), egui::DragValue::new(&mut settings.silver_strength_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("env_mod_sun_damage_label"), t!("env_mod_sun_damage_tooltip"), egui::DragValue::new(&mut settings.sun_damage_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
    });
    changed
}

fn show_castle_common_settings(ui: &mut egui::Ui, settings: &mut settings::Settings) -> bool {
    let mut changed = false;
    egui::Grid::new("castle_common_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("castle_mod_decay_rate_label"), t!("castle_mod_decay_rate_tooltip"), egui::DragValue::new(&mut settings.castle_decay_rate_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("castle_mod_blood_essence_drain_label"), t!("castle_mod_blood_essence_drain_tooltip"), egui::DragValue::new(&mut settings.castle_blood_essence_drain_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
    });
    changed
}


fn show_craft_build_modifiers(ui: &mut egui::Ui, settings: &mut settings::Settings) -> bool {
    let mut changed = false;
    egui::Grid::new("craft_build_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("craft_mod_build_cost_label"), t!("craft_mod_build_cost_tooltip"), egui::DragValue::new(&mut settings.build_cost_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_recipe_cost_label"), t!("craft_mod_recipe_cost_tooltip"), egui::DragValue::new(&mut settings.recipe_cost_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_craft_rate_label"), t!("craft_mod_craft_rate_tooltip"), egui::DragValue::new(&mut settings.craft_rate_modifier).speed(0.05).clamp_range(0.25..=6.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_research_cost_label", fallback = "Research Cost Modifier:"), t!("craft_mod_research_cost_tooltip", fallback = "Multiplier for research discovery costs (0.0 - 3.0)."), egui::DragValue::new(&mut settings.research_cost_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_refinement_cost_label"), t!("craft_mod_refinement_cost_tooltip"), egui::DragValue::new(&mut settings.refinement_cost_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_refinement_rate_label"), t!("craft_mod_refinement_rate_tooltip"), egui::DragValue::new(&mut settings.refinement_rate_modifier).speed(0.05).clamp_range(0.25..=6.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_research_time_label", fallback = "Research Time Modifier:"), t!("craft_mod_research_time_tooltip", fallback = "Multiplier for research time (0.25 - 6.0)."), egui::DragValue::new(&mut settings.research_time_modifier).speed(0.05).clamp_range(0.25..=6.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_dismantle_resource_label"), t!("craft_mod_dismantle_resource_tooltip"), egui::DragValue::new(&mut settings.dismantle_resource_modifier).speed(0.05).clamp_range(0.0..=1.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_servant_convert_rate_label"), t!("craft_mod_servant_convert_rate_tooltip"), egui::DragValue::new(&mut settings.servant_convert_rate_modifier).speed(0.05).clamp_range(0.25..=6.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("craft_mod_repair_cost_label"), t!("craft_mod_repair_cost_tooltip"), egui::DragValue::new(&mut settings.repair_cost_modifier).speed(0.05).clamp_range(0.0..=3.0).fixed_decimals(2));
    });
    changed
}


fn show_game_time_modifiers(ui: &mut egui::Ui, data: &mut settings::GameTimeModifiers) -> bool {
    let mut changed = false;
    egui::Grid::new("game_time_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("time_mod_day_duration_label"), t!("time_mod_day_duration_tooltip"), egui::DragValue::new(&mut data.day_duration_in_seconds).speed(10.0).clamp_range(60.0..=86400.0));
        changed |= add_grid_row_changed(ui, t!("time_mod_day_start_hour_label"), t!("time_mod_day_start_hour_tooltip"), egui::DragValue::new(&mut data.day_start_hour).clamp_range(0..=23));
        changed |= add_grid_row_changed(ui, t!("time_mod_day_start_min_label"), t!("time_mod_day_start_min_tooltip"), egui::DragValue::new(&mut data.day_start_minute).clamp_range(0..=59));
        changed |= add_grid_row_changed(ui, t!("time_mod_day_end_hour_label"), t!("time_mod_day_end_hour_tooltip"), egui::DragValue::new(&mut data.day_end_hour).clamp_range(0..=23));
        changed |= add_grid_row_changed(ui, t!("time_mod_day_end_min_label"), t!("time_mod_day_end_min_tooltip"), egui::DragValue::new(&mut data.day_end_minute).clamp_range(0..=59));
        changed |= add_grid_row_changed(ui, t!("time_mod_blood_moon_freq_min_label"), t!("time_mod_blood_moon_freq_min_tooltip"), egui::DragValue::new(&mut data.blood_moon_frequency_min).clamp_range(1..=255));
        changed |= add_grid_row_changed(ui, t!("time_mod_blood_moon_freq_max_label"), t!("time_mod_blood_moon_freq_max_tooltip"), egui::DragValue::new(&mut data.blood_moon_frequency_max).clamp_range(1..=255));
        changed |= add_grid_row_changed(ui, t!("time_mod_blood_moon_buff_label"), t!("time_mod_blood_moon_buff_tooltip"), egui::DragValue::new(&mut data.blood_moon_buff).speed(0.01).clamp_range(0.0..=1.0).fixed_decimals(2));
    });
    changed
}

fn show_vampire_stat_modifiers(ui: &mut egui::Ui, data: &mut settings::VampireStatModifiers) -> bool {
    let mut changed = false;
    egui::Grid::new("vampire_stats_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("vamp_stats_max_health_label"), t!("vamp_stats_max_health_tooltip"), egui::DragValue::new(&mut data.max_health_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("vamp_stats_phys_power_label"), t!("vamp_stats_phys_power_tooltip"), egui::DragValue::new(&mut data.physical_power_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("vamp_stats_spell_power_label"), t!("vamp_stats_spell_power_tooltip"), egui::DragValue::new(&mut data.spell_power_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("vamp_stats_res_power_label"), t!("vamp_stats_res_power_tooltip"), egui::DragValue::new(&mut data.resource_power_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("vamp_stats_siege_power_label", fallback = "Siege Power Modifier:"), t!("vamp_stats_siege_power_tooltip", fallback = "Multiplier for base vampire siege power (0.01 - 10.0)."), egui::DragValue::new(&mut data.siege_power_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("vamp_stats_dmg_received_label"), t!("vamp_stats_dmg_received_tooltip"), egui::DragValue::new(&mut data.damage_received_modifier).speed(0.05).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("vamp_stats_revive_cancel_label"), t!("vamp_stats_revive_cancel_tooltip"), egui::DragValue::new(&mut data.revive_cancel_delay).speed(0.1).fixed_decimals(1));
    });
    changed
}

fn show_unit_stat_modifiers(ui: &mut egui::Ui, id_prefix: &str, data: &mut settings::UnitStatModifiers) -> bool {
    let mut changed = false;
    egui::Grid::new(format!("{}_unit_stats_grid", id_prefix)).num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("unit_stats_max_health_label"), t!("unit_stats_max_health_tooltip"), egui::DragValue::new(&mut data.max_health_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("unit_stats_power_label"), t!("unit_stats_power_tooltip"), egui::DragValue::new(&mut data.power_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("unit_stats_level_increase_label"), t!("unit_stats_level_increase_tooltip"), egui::DragValue::new(&mut data.level_increase).clamp_range(0..=100));
    });
    changed
}

fn show_equipment_stat_modifiers(ui: &mut egui::Ui, data: &mut settings::EquipmentStatModifiers) -> bool {
    let mut changed = false;
    egui::Grid::new("equip_stats_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("equip_stats_max_health_label"), t!("equip_stats_max_health_tooltip"), egui::DragValue::new(&mut data.max_health_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("equip_stats_res_yield_label"), t!("equip_stats_res_yield_tooltip"), egui::DragValue::new(&mut data.resource_yield_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("equip_stats_phys_power_label"), t!("equip_stats_phys_power_tooltip"), egui::DragValue::new(&mut data.physical_power_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("equip_stats_spell_power_label"), t!("equip_stats_spell_power_tooltip"), egui::DragValue::new(&mut data.spell_power_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("equip_stats_siege_power_label"), t!("equip_stats_siege_power_tooltip"), egui::DragValue::new(&mut data.siege_power_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("equip_stats_move_speed_label", fallback = "Movement Speed Modifier (from Equip):"), t!("equip_stats_move_speed_tooltip", fallback = "Multiplier for movement speed bonus from equipment (0.01 - 10.0)."), egui::DragValue::new(&mut data.movement_speed_modifier).speed(0.05).clamp_range(0.01..=10.0).fixed_decimals(2));
    });
    changed
}

fn show_castle_stat_modifiers(ui: &mut egui::Ui, data: &mut settings::CastleStatModifiers) -> bool {
    let mut changed = false;
    egui::Grid::new("castle_stats_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("castle_stats_tick_period_label"), t!("castle_stats_tick_period_tooltip"), egui::DragValue::new(&mut data.tick_period).speed(0.1).clamp_range(0.1..=600.0).fixed_decimals(1));
        changed |= add_grid_row_changed(ui, t!("castle_stats_safetybox_label"), t!("castle_stats_safetybox_tooltip"), egui::DragValue::new(&mut data.safety_box_limit).clamp_range(0..=255));
        changed |= add_grid_row_changed(ui, t!("castle_stats_tomb_label"), t!("castle_stats_tomb_tooltip"), egui::DragValue::new(&mut data.tomb_limit).clamp_range(0..=255));
        changed |= add_grid_row_changed(ui, t!("castle_stats_eye_label"), t!("castle_stats_eye_tooltip"), egui::DragValue::new(&mut data.eye_structures_limit).clamp_range(0..=255));
        changed |= add_grid_row_changed(ui, t!("castle_stats_vermin_label"), t!("castle_stats_vermin_tooltip"), egui::DragValue::new(&mut data.vermin_nest_limit).clamp_range(0..=255));
        changed |= add_grid_row_changed(ui, t!("castle_stats_prison_label"), t!("castle_stats_prison_tooltip"), egui::DragValue::new(&mut data.prison_cell_limit).clamp_range(0..=255));
        changed |= add_grid_row_changed(ui, t!("castle_stats_castle_limit_label"), t!("castle_stats_castle_limit_tooltip"), egui::DragValue::new(&mut data.castle_limit).clamp_range(0..=255));

        ui.label(t!("castle_stats_limit_type_label")).on_hover_text(t!("castle_stats_limit_type_tooltip"));
        let limit_type = data.castle_heart_limit_type.get_or_insert(settings::CastleHeartLimitType::default());
        egui::ComboBox::from_id_source("castle_limit_type_combo")
           .selected_text(t!(&format!("enums.castle_heart_limit_type.{}", limit_type)))
           .show_ui(ui, |ui| {
               for v in CastleHeartLimitType::iter() {
                   changed |= ui.selectable_value(limit_type, v, t!(&format!("enums.castle_heart_limit_type.{}", v))).changed();
               }
           });
        ui.end_row();

        changed |= add_grid_row_changed(ui, t!("castle_stats_nethergate_label"), t!("castle_stats_nethergate_tooltip"), egui::DragValue::new(&mut data.nether_gate_limit).clamp_range(0..=255));
        changed |= add_grid_row_changed(ui, t!("castle_stats_throne_label"), t!("castle_stats_throne_tooltip"), egui::DragValue::new(&mut data.throne_of_darkness_limit).clamp_range(0..=255));
     });
    ui.separator();
    ui.label(t!("castle_stats_heart_limits_header"));
    egui::Grid::new("heart_limits_grid").num_columns(5).spacing([10.0, 4.0]).striped(true).show(ui, |ui| {
        ui.label(t!("castle_stats_heart_level", level = ""));
        ui.label(t!("castle_stats_heart_floor")).on_hover_text(t!("castle_stats_heart_floor_tooltip"));
        ui.label(t!("castle_stats_heart_servant")).on_hover_text(t!("castle_stats_heart_servant_tooltip"));
        ui.label(t!("castle_stats_heart_build")).on_hover_text(t!("castle_stats_heart_build_tooltip"));
        ui.label(t!("castle_stats_heart_height")).on_hover_text(t!("castle_stats_heart_height_tooltip"));
        ui.end_row();

        fn show_heart_level(ui: &mut egui::Ui, level: u8, limit: &mut settings::HeartLevelLimit) -> bool {
            let mut row_changed = false;
            ui.label(t!("castle_stats_heart_level", level = level));
            row_changed |= ui.add(egui::DragValue::new(&mut limit.floor_limit)).changed();
            row_changed |= ui.add(egui::DragValue::new(&mut limit.servant_limit).clamp_range(0..=35)).changed();
            row_changed |= ui.add(egui::DragValue::new(&mut limit.build_limits)).changed();
            row_changed |= ui.add(egui::DragValue::new(&mut limit.height_limit).clamp_range(0..=255)).changed();
            ui.end_row();
            row_changed
        }

        changed |= show_heart_level(ui, 1, &mut data.heart_limits.level1);
        changed |= show_heart_level(ui, 2, &mut data.heart_limits.level2);
        changed |= show_heart_level(ui, 3, &mut data.heart_limits.level3);
        changed |= show_heart_level(ui, 4, &mut data.heart_limits.level4);
        changed |= show_heart_level(ui, 5, &mut data.heart_limits.level5);
    });
    changed
}


fn show_player_interaction_settings(ui: &mut egui::Ui, data: &mut settings::PlayerInteractionSettings) -> bool {
    let mut changed = false;
    egui::Grid::new("player_interaction_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        ui.label(t!("pis_timezone_label")).on_hover_text(t!("pis_timezone_tooltip"));
        egui::ComboBox::from_id_source("timezone_combo")
            .selected_text(t!(&format!("enums.time_zone.{}", data.time_zone)))
            .show_ui(ui, |ui| {
                for v in TimeZone::iter() {
                    changed |= ui.selectable_value(&mut data.time_zone, v, t!(&format!("enums.time_zone.{}", v))).changed();
                }
            });
        ui.end_row();
    });
    ui.separator();
    ui.label(t!("pis_pvp_time_label"));
    changed |= show_start_end_time_grid(ui, "pvp_time", &mut data.vs_player_weekday_time, &mut data.vs_player_weekend_time);
    ui.separator();
    ui.label(t!("pis_castle_time_label"));
    changed |= show_start_end_time_grid(ui, "castle_time", &mut data.vs_castle_weekday_time, &mut data.vs_castle_weekend_time);
    changed
}

fn show_start_end_time_grid(
    ui: &mut egui::Ui,
    id_prefix: &str,
    weekday_data: &mut settings::StartEndTimeData,
    weekend_data: &mut settings::StartEndTimeData,
) -> bool {
    let mut changed = false;
    egui::Grid::new(format!("{}_grid", id_prefix)).num_columns(5).spacing([10.0, 4.0]).striped(true).show(ui, |ui| {
        ui.label("");
        ui.label(t!("pis_start_hour"));
        ui.label(t!("pis_start_min"));
        ui.label(t!("pis_end_hour"));
        ui.label(t!("pis_end_min"));
        ui.end_row();

        ui.label(t!("pis_weekday_label"));
        changed |= ui.add(egui::DragValue::new(&mut weekday_data.start_hour).clamp_range(0..=23)).changed();
        changed |= ui.add(egui::DragValue::new(&mut weekday_data.start_minute).clamp_range(0..=59)).changed();
        changed |= ui.add(egui::DragValue::new(&mut weekday_data.end_hour).clamp_range(0..=23)).changed();
        changed |= ui.add(egui::DragValue::new(&mut weekday_data.end_minute).clamp_range(0..=59)).changed();
        ui.end_row();

        ui.label(t!("pis_weekend_label"));
        changed |= ui.add(egui::DragValue::new(&mut weekend_data.start_hour).clamp_range(0..=23)).changed();
        changed |= ui.add(egui::DragValue::new(&mut weekend_data.start_minute).clamp_range(0..=59)).changed();
        changed |= ui.add(egui::DragValue::new(&mut weekend_data.end_hour).clamp_range(0..=23)).changed();
        changed |= ui.add(egui::DragValue::new(&mut weekend_data.end_minute).clamp_range(0..=59)).changed();
        ui.end_row();
    });
    changed
}

fn show_trader_modifiers(ui: &mut egui::Ui, data: &mut settings::TraderModifiers) -> bool {
    let mut changed = false;
    egui::Grid::new("trader_mods_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        changed |= add_grid_row_changed(ui, t!("trader_mod_stock_label"), t!("trader_mod_stock_tooltip"), egui::DragValue::new(&mut data.stock_modifier).speed(0.05).clamp_range(0.25..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("trader_mod_price_label"), t!("trader_mod_price_tooltip"), egui::DragValue::new(&mut data.price_modifier).speed(0.05).clamp_range(0.25..=10.0).fixed_decimals(2));
        changed |= add_grid_row_changed(ui, t!("trader_mod_restock_label"), t!("trader_mod_restock_tooltip"), egui::DragValue::new(&mut data.restock_timer_modifier).speed(0.05).clamp_range(0.25..=10.0).fixed_decimals(2));
    });
    changed
}

fn show_war_event_settings(ui: &mut egui::Ui, data: &mut settings::WarEventGameSettings) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        changed |= ui.checkbox(&mut data.enable_war_events, t!("wes_enable_label")).changed();
        changed |= ui.checkbox(&mut data.enable_incursions, t!("wes_enable_incursions_label")).changed();
    });
    ui.horizontal(|ui| {
        changed |= ui.checkbox(&mut data.enable_major_incursions, t!("wes_enable_major_label")).changed();
        changed |= ui.checkbox(&mut data.enable_minor_incursions, t!("wes_enable_minor_label")).changed();
    });
    ui.separator();
    egui::Grid::new("war_event_grid").num_columns(2).spacing([40.0, 4.0]).striped(true).show(ui, |ui| {
        ui.label(t!("wes_interval_label")).on_hover_text(t!("wes_interval_tooltip"));
        egui::ComboBox::from_id_source("war_interval_combo")
            .selected_text(t!(&format!("enums.war_event_interval.{}", data.interval)))
            .show_ui(ui, |ui| {
                for variant in WarEventInterval::iter() {
                    changed |= ui.selectable_value(&mut data.interval, variant, t!(&format!("enums.war_event_interval.{}", variant))).changed();
                }
            });
        ui.end_row();

        ui.label(t!("wes_major_duration_label")).on_hover_text(t!("wes_major_duration_tooltip"));
        egui::ComboBox::from_id_source("war_major_duration_combo")
             .selected_text(t!(&format!("enums.war_event_duration.{}", data.major_duration)))
            .show_ui(ui, |ui| {
                for variant in WarEventDuration::iter() {
                    changed |= ui.selectable_value(&mut data.major_duration, variant, t!(&format!("enums.war_event_duration.{}", variant))).changed();
                }
            });
        ui.end_row();

        ui.label(t!("wes_minor_duration_label")).on_hover_text(t!("wes_minor_duration_tooltip"));
        egui::ComboBox::from_id_source("war_minor_duration_combo")
            .selected_text(t!(&format!("enums.war_event_duration.{}", data.minor_duration)))
            .show_ui(ui, |ui| {
                for variant in WarEventDuration::iter() {
                    changed |= ui.selectable_value(&mut data.minor_duration, variant, t!(&format!("enums.war_event_duration.{}", variant))).changed();
                }
            });
        ui.end_row();
    });
    ui.separator();
    ui.label(t!("wes_time_active_label"));
    changed |= show_start_end_time_grid(ui, "war_event_active_time", &mut data.week_day_time, &mut data.weekend_time);
    changed
}

fn show_vblood_settings(
    ui: &mut egui::Ui,
    vblood_settings: &mut Vec<settings::VBloodUnitSetting>,
    new_vblood_id_to_add: &mut VBloodUnitId,
) -> bool {
    let mut changed = false;
    let mut remove_index = None;

    ui.horizontal(|ui| {
        ui.label(t!("add_vblood_setting_for"));
        egui::ComboBox::from_id_source("new_vblood_select")
            .selected_text(t!(&format!("enums.vblood_unit_id.{}", new_vblood_id_to_add)))
            .show_ui(ui, |ui| {
                for unit_id in VBloodUnitId::iter() {
                    let already_added = vblood_settings.iter().any(|s| s.unit_id == unit_id);
                    let label_text = t!(&format!("enums.vblood_unit_id.{}", unit_id));

                    let response = ui.add_enabled(
                        !already_added,
                        egui::SelectableLabel::new(*new_vblood_id_to_add == unit_id, label_text),
                    );
                    let clicked = response.clicked();
                    if already_added {
                        response.on_disabled_hover_text(t!("already_added_tooltip"));
                    }
                    if clicked {
                        *new_vblood_id_to_add = unit_id;
                    }
                }
            });

        let can_add = !vblood_settings.iter().any(|s| s.unit_id == *new_vblood_id_to_add);
        if ui.add_enabled(can_add, egui::Button::new(t!("add_button"))).clicked() {
            vblood_settings.push(settings::VBloodUnitSetting {
                unit_id: *new_vblood_id_to_add,
                unit_level: 0,
                default_unlocked: false,
            });
            changed = true;
        }
    });
    ui.separator();

    ui.label(t!("existing_vblood_settings"));
    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
        for (i, vblood) in vblood_settings.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(t!(&format!("enums.vblood_unit_id.{}", vblood.unit_id)));
                ui.add_space(10.0);
                ui.label(t!("vblood_level_label"));
                changed |= ui.add(egui::DragValue::new(&mut vblood.unit_level).clamp_range(0..=255)).changed();
                changed |= ui.checkbox(&mut vblood.default_unlocked, t!("vblood_unlocked_label")).changed();
                let remove_button_text = t!("remove_button_tooltip", item = t!(&format!("enums.vblood_unit_id.{}", vblood.unit_id)));
                if ui.button("X").on_hover_text(remove_button_text).clicked() {
                    remove_index = Some(i);
                    changed = true;
                }
            });
            ui.separator();
        }
    });

    if let Some(index) = remove_index {
        vblood_settings.remove(index);
    }
    changed
}


fn show_unlocked_achievements(
    ui: &mut egui::Ui,
    list: &mut Vec<AchievementId>,
    new_achievement_to_add: &mut AchievementId,
) -> bool {
    let mut changed = false;
    let mut remove_index = None;

    ui.horizontal(|ui| {
        ui.label(t!("add_achievement"));
        egui::ComboBox::from_id_source("new_achievement_select")
            .selected_text(t!(&format!("enums.achievement_id.{}", new_achievement_to_add)))
            .show_ui(ui, |ui| {
                for ach_id in AchievementId::iter() {
                    let already_added = list.contains(&ach_id);
                    let label_text = t!(&format!("enums.achievement_id.{}", ach_id));

                    let response = ui.add_enabled(
                        !already_added,
                        egui::SelectableLabel::new(*new_achievement_to_add == ach_id, label_text),
                    );
                    let clicked = response.clicked();
                    if already_added {
                        response.on_disabled_hover_text(t!("already_added_tooltip"));
                    }
                    if clicked {
                        *new_achievement_to_add = ach_id;
                    }
                }
            });

        let can_add = !list.contains(new_achievement_to_add);
        if ui.add_enabled(can_add, egui::Button::new(t!("add_button"))).clicked() {
            list.push(*new_achievement_to_add);
            changed = true;
        }
    });
    ui.separator();

    ui.label(t!("unlocked_achievements"));
    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
        for (i, ach_id) in list.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(t!(&format!("enums.achievement_id.{}", ach_id)));
                 let remove_button_text = t!("remove_achievement_tooltip", item = t!(&format!("enums.achievement_id.{}", ach_id)));
                if ui.button("X").on_hover_text(remove_button_text).clicked() {
                    remove_index = Some(i);
                    changed = true;
                }
            });
            ui.separator();
        }
    });

    if let Some(index) = remove_index {
        list.remove(index);
    }
    changed
}


fn show_unlocked_research(
    ui: &mut egui::Ui,
    list: &mut Vec<ResearchId>,
    new_research_to_add: &mut ResearchId,
) -> bool {
    let mut changed = false;
    let mut remove_index = None;

    ui.horizontal(|ui| {
        ui.label(t!("add_research"));
        egui::ComboBox::from_id_source("new_research_select")
            .selected_text(t!(&format!("enums.research_id.{}", new_research_to_add)))
            .show_ui(ui, |ui| {
                for res_id in ResearchId::iter() {
                    let already_added = list.contains(&res_id);
                    let label_text = t!(&format!("enums.research_id.{}", res_id));

                    let response = ui.add_enabled(
                        !already_added,
                        egui::SelectableLabel::new(*new_research_to_add == res_id, label_text),
                    );
                    let clicked = response.clicked();
                     if already_added {
                        response.on_disabled_hover_text(t!("already_added_tooltip"));
                    }
                    if clicked {
                        *new_research_to_add = res_id;
                    }
                }
            });

        let can_add = !list.contains(new_research_to_add);
        if ui.add_enabled(can_add, egui::Button::new(t!("add_button"))).clicked() {
            list.push(*new_research_to_add);
            changed = true;
        }
    });
    ui.separator();

    ui.label(t!("unlocked_research"));
    egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
        for (i, res_id) in list.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(t!(&format!("enums.research_id.{}", res_id)));
                let remove_button_text = t!("remove_research_tooltip", item = t!(&format!("enums.research_id.{}", res_id)));
                if ui.button("X").on_hover_text(remove_button_text).clicked() {
                    remove_index = Some(i);
                    changed = true;
                }
            });
            ui.separator();
        }
    });

    if let Some(index) = remove_index {
        list.remove(index);
    }
    changed
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_min_inner_size([600.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "V Rising Settings Editor",
        options,
        Box::new(|cc| Box::new(VrisingEditorApp::new(cc))),
    )
}
