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
pub(super) struct SettingDialogControls {
    layout: SettingDialogLayout,

    pub(super) font_normal: nwg::Font,

    pub(super) icon: nwg::Icon,
    pub(super) window: nwg::Window,

    pub(super) name_label: nwg::Label,
    pub(super) name_input: nwg::TextInput,
    pub(super) default_value_label: nwg::Label,
    pub(super) default_value_input: nwg::TextInput,
    pub(super) dsn_value_label: nwg::Label,
    pub(super) dsn_value_input: nwg::TextInput,
    pub(super) fs_path_button: nwg::Button,
    pub(super) fs_path_chooser: nwg::FileDialog,
    pub(super) bool_value_checkbox: nwg::CheckBox,
    pub(super) description_label: nwg::Label,

    pub(super) apply_button: nwg::Button,
    pub(super) delete_button: nwg::Button,
    pub(super) close_button: nwg::Button,
}

impl ui::Controls for SettingDialogControls {
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
            .size((480, 210))
            .icon(Some(&self.icon))
            .center(true)
            .title("Change Setting")
            .build(&mut self.window)?;

        nwg::Label::builder()
            .text("Setting name:")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.name_label)?;
        nwg::TextInput::builder()
            .readonly(true)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.name_input)?;

        nwg::Label::builder()
            .text("Default value:")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.default_value_label)?;
        nwg::TextInput::builder()
            .readonly(true)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.default_value_input)?;

        nwg::Label::builder()
            .text("DSN value:")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.dsn_value_label)?;
        nwg::TextInput::builder()
            .flags(nwg::TextInputFlags::VISIBLE)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.dsn_value_input)?;
        nwg::Button::builder()
            .text("Choose")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.fs_path_button)?;
        nwg::FileDialog::builder()
            .title("Choose file")
            .action(nwg::FileDialogAction::Open)
            .build(&mut self.fs_path_chooser)?;

        nwg::CheckBox::builder()
            .check_state(nwg::CheckBoxState::Unchecked)
            .text("Set value ON/OFF")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.bool_value_checkbox)?;

        nwg::Label::builder()
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.description_label)?;

        nwg::Button::builder()
            .text("Apply value")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.apply_button)?;
        nwg::Button::builder()
            .text("Delete")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.delete_button)?;
        nwg::Button::builder()
            .text("Cancel")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;

        self.layout.build(&self)?;

        Ok(())
    }

    fn update_tab_order(&self) {
        ui::tab_order_builder()
            .control(&self.name_input)
            .control(&self.default_value_input)
            .control(&self.dsn_value_input)
            .control(&self.fs_path_button)
            .control(&self.bool_value_checkbox)
            .control(&self.apply_button)
            .control(&self.delete_button)
            .control(&self.close_button)
            .build();
    }
}
