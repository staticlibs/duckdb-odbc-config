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

#[derive(Default)]
pub(super) struct AddDsnDialogEvents {
    pub(super) events: Vec<ui::Event<AddDsnDialog>>
}

impl ui::Events<AddDsnDialogControls> for AddDsnDialogEvents {
    fn build(&mut self, c: &AddDsnDialogControls) -> Result<(), nwg::NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(AddDsnDialog::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnResizeEnd)
            .handler(AddDsnDialog::on_resize)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.name_input)
            .event(nwg::Event::OnTextInput)
            .handler(AddDsnDialog::on_name_input)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.dbpath_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AddDsnDialog::on_choose_db_file)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.use_memory_db_checkbox)
            .event(nwg::Event::OnButtonClick)
            .handler(AddDsnDialog::on_memory_checkbox_changed)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.save_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AddDsnDialog::on_save_button)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.cancel_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AddDsnDialog::close)
            .build(&mut self.events)?;

        Ok(())
    }
}
