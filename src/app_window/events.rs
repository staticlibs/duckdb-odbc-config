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
pub(super) struct AppWindowEvents {
    pub(super) events: Vec<ui::Event<AppWindow>>
}

impl ui::Events<AppWindowControls> for AppWindowEvents {
    fn build(&mut self, c: &AppWindowControls) -> Result<(), nwg::NwgError> {
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnWindowClose)
            .handler(AppWindow::close)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnResizeEnd)
            .handler(AppWindow::on_resize)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.window)
            .event(nwg::Event::OnKeyEnter)
            .handler(AppWindow::on_filter_button)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.file_add_dsn_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_add_dsn_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.file_exit_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.help_about_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_about_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.help_website_menu_item)
            .event(nwg::Event::OnMenuItemSelected)
            .handler(AppWindow::open_website)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.dsn_combo)
            .event(nwg::Event::OnComboxBoxSelection)
            .handler(AppWindow::on_dsn_changed)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.filter_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::on_filter_button)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.settings_view)
            .event(nwg::Event::OnListViewColumnClick)
            .handler(AppWindow::on_settings_view_sort)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.settings_view)
            .event(nwg::Event::OnListViewDoubleClick)
            .handler(AppWindow::open_setting_dialog)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.new_dsn_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::open_add_dsn_dialog)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.reload_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::open_load_dialog)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(AppWindow::close)
            .build(&mut self.events)?;

        ui::event_builder()
            .control(&c.about_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_about_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.load_settings_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_load_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.setting_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_setting_dialog)
            .build(&mut self.events)?;
        ui::event_builder()
            .control(&c.add_dsn_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(AppWindow::await_add_dsn_dialog)
            .build(&mut self.events)?;

        Ok(())
    }
}
