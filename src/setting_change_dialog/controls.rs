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
pub(super) struct SettingChangeDialogControls {
    layout: SettingChangeDialogLayout,

    pub(super) font_normal: nwg::Font,

    pub(super) icon: nwg::Icon,
    pub(super) window: nwg::Window,

    pub(super) progress_bar: nwg::ProgressBar,
    pub(super) label: nwg::Label,
    pub(super) details_box: nwg::TextBox,
    pub(super) copy_clipboard_button: nwg::Button,
    pub(super) close_button: nwg::Button,

    pub(super) change_notice: ui::SyncNotice,
}

impl ui::Controls for SettingChangeDialogControls {
    fn build(&mut self) -> Result<(), nwg::NwgError> {
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;

        nwg::Icon::builder()
            .source_embed(Some(&nwg::EmbedResource::load(None)
                .expect("Error loading embedded resource")))
            .source_embed_id(2)
            .build(&mut self.icon)?;

        nwg::Window::builder()
            .size((320, 200))
            .icon(Some(&self.icon))
            .center(true)
            .title("Apply Setting Change")
            .build(&mut self.window)?;

        nwg::ProgressBar::builder()
            .flags(nwg::ProgressBarFlags::VISIBLE | nwg::ProgressBarFlags::MARQUEE)
            .marquee(true)
            .marquee_update(30)
            .range(0..1)
            .parent(&self.window)
            .build(&mut self.progress_bar)?;

        nwg::Label::builder()
            .text("Applying ...")
            .flags(nwg::LabelFlags::VISIBLE | nwg::LabelFlags::ELIPSIS)
            .font(Some(&self.font_normal))
            .v_align(nwg::VTextAlign::Top)
            .parent(&self.window)
            .build(&mut self.label)?;

        nwg::TextBox::builder()
            .text("Details pending ...")
            .font(Some(&self.font_normal))
            .readonly(true)
            .parent(&self.window)
            .build(&mut self.details_box)?;

        nwg::Button::builder()
            .text("Copy to clipboard")
            .enabled(false)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.copy_clipboard_button)?;

        nwg::Button::builder()
            .text("Close")
            .enabled(false)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;

        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.change_notice)?;

        self.layout.build(&self)?;

        Ok(())
    }

    fn update_tab_order(&self) {
        ui::tab_order_builder()
            .control(&self.copy_clipboard_button)
            .control(&self.close_button)
            .build();
    }
}
