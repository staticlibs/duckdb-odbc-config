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

const LABEL_WIDTH_PT: u32 = 80;

#[derive(Default)]
pub(super) struct AddDsnDialogLayout {
    root_layout: nwg::FlexboxLayout,
    name_layout: nwg::FlexboxLayout,
    dsn_type_layout: nwg::FlexboxLayout,
    dbpath_layout: nwg::FlexboxLayout,
    use_memory_layout: nwg::FlexboxLayout,
    spacer_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,
}

impl ui::Layout<AddDsnDialogControls> for AddDsnDialogLayout {
    fn build(&self, c: &AddDsnDialogControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.name_label)
            .child_size(ui::size_builder()
                .width_pt(LABEL_WIDTH_PT)
                .height_input_form_row()
                .build())
            .child(&c.name_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.name_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.dsn_type_label)
            .child_size(ui::size_builder()
                .width_pt(LABEL_WIDTH_PT)
                .height_input_form_row()
                .build())
            .child(&c.dsn_type_combo)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.dsn_type_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.dbpath_label)
            .child_size(ui::size_builder()
                .width_pt(LABEL_WIDTH_PT)
                .height_input_form_row()
                .build())
            .child(&c.dbpath_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .child(&c.dbpath_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .build_partial(&self.dbpath_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.use_memory_db_checkbox)
            .child_size(ui::size_builder()
                .width_auto()
                .height_input_form_row()
                .build())
            .child_flex_grow(1.0)
            .child_margin(ui::margin_builder()
                .start_pt(85)
                .build())
            .build_partial(&self.use_memory_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .build_partial(&self.spacer_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .justify_content(ui::JustifyContent::FlexEnd)
            .auto_spacing(None)
            .child(&c.save_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child(&c.cancel_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .build_partial(&self.buttons_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Column)
            .child_layout(&self.name_layout)
            .child_layout(&self.dsn_type_layout)
            .child_layout(&self.dbpath_layout)
            .child_layout(&self.use_memory_layout)
            .child_layout(&self.spacer_layout)
            .child_flex_grow(1.0)
            .child_layout(&self.buttons_layout)
            .build(&self.root_layout)?;

        Ok(())
    }
}
