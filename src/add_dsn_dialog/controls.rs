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
pub(super) struct AddDsnDialogControls {
    layout: AddDsnDialogLayout,

    pub(super) font_normal: nwg::Font,

    pub(super) icon: nwg::Icon,
    pub(super) window: nwg::Window,

    pub(super) name_label: nwg::Label,
    pub(super) name_input: nwg::TextInput,
    pub(super) dsn_type_label: nwg::Label,
    pub(super) dsn_type_combo: nwg::ComboBox<String>,
    pub(super) dbpath_label: nwg::Label,
    pub(super) dbpath_input: nwg::TextInput,
    pub(super) dbpath_button: nwg::Button,
    pub(super) dbpath_chooser: nwg::FileDialog,
    pub(super) use_memory_db_checkbox: nwg::CheckBox,
    pub(super) init_label: nwg::Label,
    pub(super) init_input: nwg::TextInput,
    pub(super) init_button: nwg::Button,
    pub(super) init_chooser: nwg::FileDialog,

    pub(super) save_button: nwg::Button,
    pub(super) cancel_button: nwg::Button,
}

impl ui::Controls for AddDsnDialogControls {

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
            .size((380, 210))
            .icon(Some(&self.icon))
            .center(true)
            .title("Add Data Source")
            .build(&mut self.window)?;

        nwg::Label::builder()
            .text("Name:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .v_align(nwg::VTextAlign::Center)
            .parent(&self.window)
            .build(&mut self.name_label)?;
        nwg::TextInput::builder()
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.name_input)?;

        nwg::Label::builder()
            .text("Type:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .v_align(nwg::VTextAlign::Center)
            .parent(&self.window)
            .build(&mut self.dsn_type_label)?;
        nwg::ComboBox::builder()
            .font(Some(&self.font_normal))
            .collection(vec!(
                "User".to_string(),
                "System".to_string(),
            ))
            .selected_index(Some(0))
            .parent(&self.window)
            .build(&mut self.dsn_type_combo)?;

        nwg::Label::builder()
            .text("Database:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .v_align(nwg::VTextAlign::Center)
            .parent(&self.window)
            .build(&mut self.dbpath_label)?;
        nwg::TextInput::builder()
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.dbpath_input)?;
        nwg::Button::builder()
            .text("Choose")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.dbpath_button)?;
        nwg::FileDialog::builder()
            .title("Choose database file")
            .action(nwg::FileDialogAction::Open)
            .build(&mut self.dbpath_chooser)?;

        nwg::CheckBox::builder()
            .check_state(nwg::CheckBoxState::Unchecked)
            .text("Use in-memory database")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.use_memory_db_checkbox)?;

        nwg::Label::builder()
            .text("Init SQL file:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .v_align(nwg::VTextAlign::Center)
            .parent(&self.window)
            .build(&mut self.init_label)?;
        nwg::TextInput::builder()
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.init_input)?;
        nwg::Button::builder()
            .text("Choose")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.init_button)?;
        nwg::FileDialog::builder()
            .title("Choose session init SQL file")
            .action(nwg::FileDialogAction::Open)
            .build(&mut self.init_chooser)?;

        nwg::Button::builder()
            .text("Save")
            .font(Some(&self.font_normal))
            .enabled(false)
            .parent(&self.window)
            .build(&mut self.save_button)?;
        nwg::Button::builder()
            .text("Cancel")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.cancel_button)?;

        self.layout.build(&self)?;

        Ok(())
    }

    fn update_tab_order(&self) {
        ui::tab_order_builder()
            .control(&self.name_input)
            .control(&self.dsn_type_combo)
            .control(&self.dbpath_input)
            .control(&self.dbpath_button)
            .control(&self.use_memory_db_checkbox)
            .control(&self.init_input)
            .control(&self.init_button)
            .control(&self.save_button)
            .control(&self.cancel_button)
            .build();
    }
}
