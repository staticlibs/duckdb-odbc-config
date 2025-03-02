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
use nwg::NwgError;

#[derive(Default)]
pub(super) struct SettingDialogEvents {
    pub(super) events: Vec<ui::Event<SettingDialog>>
}

impl ui::Events<SettingDialogControls> for SettingDialogEvents {
    fn build(&mut self, c: &SettingDialogControls) -> Result<(), NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(SettingDialog::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnResizeEnd)
            .handler(SettingDialog::on_resize)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.change_button)
            .event(nwg::Event::OnButtonClick)
            .handler(SettingDialog::open_change_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(SettingDialog::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.new_value_input)
            .event(nwg::Event::OnTextInput)
            .handler(SettingDialog::on_new_value_change)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.change_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(SettingDialog::await_change_dialog)
            .build(&mut self.events)?;

        Ok(())
    }
}
