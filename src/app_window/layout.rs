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

const BOTTOM_MARGIN_PT: u32 = 22;

#[derive(Default)]
pub(super) struct AppWindowLayout {
    root_layout: nwg::FlexboxLayout,
    filter_panel_layout: nwg::FlexboxLayout,
    settings_view_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,
}

impl ui::Layout<AppWindowControls> for AppWindowLayout {
    fn build(&self, c: &AppWindowControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .child(&c.dsn_label)
            .child_size(ui::size_builder()
                .width_pt(35)
                .height_input_form_row()
                .build())
            .child_margin(ui::margin_builder()
                .top_pt(1)
                .build())
            .child(&c.dsn_combo)
            .child_size(ui::size_builder()
                .width_pt(120)
                .height_input_form_row()
                .build())
            .child_margin(ui::margin_builder()
                .top_pt(2)
                .build())
            .child(&c.filter_input)
            .child_size(ui::size_builder()
                .width_auto()
                .height_input_form_row()
                .build())
            .child_margin(ui::margin_builder()
                .start_default()
                .top_pt(3)
                .build())
            .child_flex_grow(1.0)
            .child(&c.filter_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_default()
                .build())
            .auto_spacing(None)
            .build_partial(&self.filter_panel_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .child(&c.settings_view)
            .child_flex_grow(1.0)
            .auto_spacing(None)
            .build_partial(&self.settings_view_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .justify_content(ui::JustifyContent::FlexEnd)
            .auto_spacing(None)
            .child(&c.new_dsn_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .bottom_pt(BOTTOM_MARGIN_PT)
                .build())
            .child(&c.delete_dsn_button)
            .child_size(ui::size_builder()
                .width_button_wide()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .bottom_pt(BOTTOM_MARGIN_PT)
                .start_pt(5)
                .build())
            .child(&c.reload_button)
            .child_size(ui::size_builder()
                .width_button_xwide()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .bottom_pt(BOTTOM_MARGIN_PT)
                .start_pt(5)
                .build())
            .child(&c.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .bottom_pt(BOTTOM_MARGIN_PT)
                .build())
            .build_partial(&self.buttons_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Column)
            .child_layout(&self.filter_panel_layout)
            .child_layout(&self.settings_view_layout)
            .child_flex_grow(1.0)
            .child_layout(&self.buttons_layout)
            .build(&self.root_layout)?;

        Ok(())
    }
}
