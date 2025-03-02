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
use crate::common::DsnType;

#[derive(Default)]
pub struct AddDsnDialog {
    pub(super) c: AddDsnDialogControls,

    added_dsn: AddDsnDialogResult,
    args: AddDsnDialogArgs,
}

impl AddDsnDialog {

    pub(super) fn on_name_input(&mut self, _: nwg::EventData) {
        self.update_save_button_state();
    }

    pub(super) fn on_choose_db_file(&mut self, _: nwg::EventData) {
        if let Ok(dir) = std::env::current_dir() {
            if let Some(d) = dir.to_str() {
                let _ = self.c.dbpath_chooser.set_default_folder(d);
            }
        }

        if self.c.dbpath_chooser.run(Some(&self.c.window)) {
            self.c.dbpath_input.set_text("");
            if let Ok(file) = self.c.dbpath_chooser.get_selected_item() {
                let fpath_st = file.to_string_lossy().to_string();
                self.c.dbpath_input.set_text(&fpath_st);
                self.update_save_button_state();
            }
        }
    }

    pub(super) fn on_memory_checkbox_changed(&mut self, _: nwg::EventData) {
        let checked = self.c.use_memory_db_checkbox.check_state() == nwg::CheckBoxState::Checked;
        if checked {
            self.c.dbpath_input.set_text("");
            self.c.dbpath_input.set_readonly(true);
            self.c.dbpath_button.set_enabled(false);
        } else {
            self.c.dbpath_input.set_readonly(false);
            self.c.dbpath_button.set_enabled(true);
        }
        self.update_save_button_state();
    }

    pub(super) fn on_save_button(&mut self, _: nwg::EventData) {
        let name = self.c.name_input.text();
        if name.is_empty() {
            return;
        }
        let in_memory = self.c.use_memory_db_checkbox.check_state() == nwg::CheckBoxState::Checked;
        let dbpath = if in_memory {
            ":memory:".to_string()
        } else {
            self.c.dbpath_input.text()
        };
        if dbpath.is_empty() {
            return;
        }
        let dsn_type_opt = self.c.dsn_type_combo.selection_string();
        let dsn_type_st = dsn_type_opt.unwrap_or("".to_string());
        if dsn_type_st.is_empty() {
            return;
        }
        let dsn_type = if "System" == dsn_type_st {
            DsnType::SYSTEM
        } else {
            DsnType::USER
        };
        match registry::create_dsn(dsn_type, &name, &dbpath) {
            Ok(()) => {
                self.added_dsn = AddDsnDialogResult::success(&name);
                self.close(nwg::EventData::NoData)
            },
            Err(e) => ui::message_box_error(&format!(
                "Cannot create Data Source, name: {}, type: {}, DB path: {}, message: {}", &name, &dsn_type_st, &dbpath, e))
        }
    }

    fn update_save_button_state(&mut self) {
        let has_name = self.c.name_input.text().len() > 0;
        let has_db = self.c.dbpath_input.text().len() > 0;
        let in_memory = self.c.use_memory_db_checkbox.check_state() == nwg::CheckBoxState::Checked;
        let can_save = has_name && (has_db || in_memory);
        self.c.save_button.set_enabled(can_save);
    }
}

impl ui::PopupDialog<AddDsnDialogArgs, AddDsnDialogResult> for AddDsnDialog {
    fn popup(args: AddDsnDialogArgs) -> ui::PopupJoinHandle<AddDsnDialogResult> {
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
       self.added_dsn = AddDsnDialogResult::cancelled()
    }

    fn result(&mut self) -> AddDsnDialogResult {
        self.added_dsn.clone()
    }

    fn close(&mut self, _: nwg::EventData) {
        self.args.notify_parent();
        self.c.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }

    fn on_resize(&mut self, _: nwg::EventData) {
        self.c.update_tab_order();
    }
}

