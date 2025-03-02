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
pub(super) struct SettingDialogLayout {
    root_layout: nwg::FlexboxLayout,
    name_layout: nwg::FlexboxLayout,
    default_value_layout: nwg::FlexboxLayout,
    dsn_value_layout: nwg::FlexboxLayout,
    bool_value_layout: nwg::FlexboxLayout,
    description_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,
}

impl ui::Layout<SettingDialogControls> for SettingDialogLayout {
    fn build(&self, c: &SettingDialogControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.name_label)
            .child_size(ui::size_builder()
                .width_label_normal()
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
            .child(&c.default_value_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&c.default_value_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.default_value_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.dsn_value_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&c.dsn_value_input)
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
            .build_partial(&self.dsn_value_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.bool_value_checkbox)
            .child_size(ui::size_builder()
                .width_auto()
                .height_input_form_row()
                .build())
            .child_flex_grow(1.0)
            .child_margin(ui::margin_builder()
                .start_no_label_normal()
                .build())
            .build_partial(&self.bool_value_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.description_label)
            .child_size(ui::size_builder()
                .width_auto()
                .height_input_form_row()
                .build())
            .child_margin(ui::margin_builder()
                .top_pt(10)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.description_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .justify_content(ui::JustifyContent::FlexEnd)
            .auto_spacing(None)
            .child(&c.apply_button)
            .child_size(ui::size_builder()
                .width_button_wide()
                .height_button()
                .build())
            .child(&c.delete_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child(&c.close_button)
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
            .child_layout(&self.default_value_layout)
            .child_layout(&self.dsn_value_layout)
            .child_layout(&self.bool_value_layout)
            .child_layout(&self.description_layout)
            .child_flex_grow(1.0)
            .child_layout(&self.buttons_layout)
            .build(&self.root_layout)?;

        Ok(())
    }
}
