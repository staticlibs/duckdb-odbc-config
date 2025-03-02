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
    change_join_handle: ui::PopupJoinHandle<SettingChangeDialogResult>,
}

impl SettingDialog {
   pub(super) fn open_change_dialog(&mut self, _: nwg::EventData) {
       self.c.window.set_enabled(false);
       let value = self.c.new_value_input.text().trim().to_string();
       let args = SettingChangeDialogArgs::new(
           &self.c.change_notice, self.args.setting.name.clone(), value);
       self.change_join_handle = SettingChangeDialog::popup(args);
   }

    pub(super) fn await_change_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.change_notice.receive();
        let res = self.change_join_handle.join();
        if res.success {
            self.c.current_value_input.set_text(&res.effective_value);
            self.result = SettingDialogResult::success(self.args.row_idx, res.effective_value.clone());
        } else {
            self.result = SettingDialogResult::failure();
        }
        ui::shake_window(&self.c.window);
        self.c.update_tab_order();
    }

    pub(super) fn on_new_value_change(&mut self, _: nwg::EventData) {
        let cur_val = self.c.current_value_input.text();
        let new_val = self.c.new_value_input.text();
        if new_val != cur_val {
            self.c.change_button.set_enabled(true);
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
        self.c.name_input.set_text(&self.args.setting.name);
        // todo
        self.c.current_value_input.set_text(&self.args.setting.default_value);
        self.c.new_value_input.set_text(&self.args.setting.default_value);
        let desc_text = ui::wrap_label_text(&self.args.setting.description, 65);
        self.c.description_label.set_text(&desc_text);
        self.c.change_button.set_enabled(false);
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
