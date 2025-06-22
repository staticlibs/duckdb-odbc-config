/*
 * Copyright 2025, DuckDB Labs
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;
use nwg::EventData;

#[derive(Default)]
pub struct SettingDialog {
    pub(super) c: SettingDialogControls,

    args: SettingDialogArgs,
    result: SettingDialogResult,
}

impl SettingDialog {
    pub(super) fn on_apply_button(&mut self, _: nwg::EventData) {
        let value = self.c.dsn_value_input.text().trim().to_string();
        let dsn = &self.args.dsn;
        let st_name = &self.args.setting.name;
        match registry::set_dsn_value(dsn.dsn_type.clone(), &dsn.name, st_name, &value) {
            Ok(_) => {
                self.result = SettingDialogResult::success();
                self.close(nwg::EventData::NoData)
            },
            Err(e) => ui::message_box_error(&format!(
                "Error setting value, DSN: '{}', setting name: '{}', value: '{}', message: '{}'", dsn.name, st_name, value, e))
        }
    }

    pub(super) fn on_delete_button(&mut self, _: nwg::EventData) {
        let dsn = &self.args.dsn;
        let st_name = &self.args.setting.name;
        let confirmed = ui::message_box_warning_yn(&format!(
            "Value: '{}' will be deleted from DSN: '{}'. Would you like to proceed?", st_name, dsn.name));
        if !confirmed {
            return;
        }
        match registry::delete_dsn_value(dsn.dsn_type.clone(), &dsn.name, st_name) {
            Ok(_) => {
                self.result = SettingDialogResult::success();
                self.close(nwg::EventData::NoData)
            },
            Err(e) => ui::message_box_error(&format!(
                "Error deleting value, DSN: '{}', setting name: '{}', message: '{}'", dsn.name, st_name, e))
        }
    }

    pub(super) fn on_bool_value_change(&mut self, _: nwg::EventData) {
        let checked = self.c.bool_value_checkbox.check_state() == nwg::CheckBoxState::Checked;
        let value = if checked {
            "true"
        } else {
            "false"
        };
        self.c.dsn_value_input.set_text(&value);
    }

    pub(super) fn on_choose_db_file(&mut self, _: nwg::EventData) {
        if let Ok(dir) = std::env::current_dir() {
            if let Some(d) = dir.to_str() {
                let _ = self.c.fs_path_chooser.set_default_folder(d);
            }
        }
        if self.c.fs_path_chooser.run(Some(&self.c.window)) {
            if let Ok(file) = self.c.fs_path_chooser.get_selected_item() {
                let fpath_st = file.to_string_lossy().to_string();
                self.c.dsn_value_input.set_text(&fpath_st);
            }
        }
    }
}

impl ui::PopupDialog<SettingDialogArgs, SettingDialogResult> for SettingDialog {
    fn popup(args: SettingDialogArgs) -> PopupJoinHandle<SettingDialogResult> {
        let join_handle = thread::spawn(move || {
            let data = Self {
                args,
                ..Default::default()
            };
            let mut dialog = Self::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            dialog.result()
        });
        ui::PopupJoinHandle::from(join_handle)
    }

    fn init(&mut self) {
        let st = &self.args.setting;
        self.c.name_input.set_text(&st.name);
        self.c.default_value_input.set_text(&st.default_value);
        if let Some(dsn_st) = self.args.dsn.settings.iter().find(|s| s.name == st.name) {
            self.c.dsn_value_input.set_text(&dsn_st.value);
            self.c.delete_button.set_enabled(true);
        } else {
            self.c.delete_button.set_enabled(false);
        }
        if "BOOLEAN" == st.input_type {
            self.c.dsn_value_input.set_readonly(true);
            self.c.bool_value_checkbox.set_enabled(true);
        } else {
            self.c.dsn_value_input.set_readonly(false);
            self.c.bool_value_checkbox.set_enabled(false);
        }
        if registry::DATABASE_SETTING_NAME == st.name || registry::SESSION_INIT_SQL_FILE_SETTING_NAME == st.name {
            self.c.fs_path_button.set_enabled(true);
            self.c.delete_button.set_enabled(false);
        } else {
            self.c.fs_path_button.set_enabled(false);
        }
        let desc_text = ui::wrap_label_text(&self.args.setting.description, 65);
        self.c.description_label.set_text(&desc_text);
        self.result = SettingDialogResult::failure();
        ui::shake_window(&self.c.window);
    }

    fn result(&mut self) -> SettingDialogResult {
        self.result.clone()
    }

    fn close(&mut self, _: nwg::EventData) {
        self.args.notify_parent();
        self.c.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }

    fn on_resize(&mut self, _: EventData) {
        self.c.update_tab_order();
    }
}
