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
pub(super) struct SettingChangeDialogLayout {
    root_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,
}

impl ui::Layout<SettingChangeDialogControls> for SettingChangeDialogLayout {
    fn build(&self, c: &SettingChangeDialogControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .justify_content(ui::JustifyContent::FlexEnd)
            .auto_spacing(None)

            .child(&c.copy_clipboard_button)
            .child_size(ui::size_builder()
                .width_button_xwide()
                .height_button()
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

            .child(&c.progress_bar)
            .child_size(ui::size_builder()
                .height_pt(30)
                .width_auto()
                .build())
            .child_align_self(ui::AlignSelf::Stretch)

            .child(&c.label)
            .child_size(ui::size_builder()
                .height_pt(10)
                .width_auto()
                .build())
            .child_align_self(ui::AlignSelf::Stretch)

            .child(&c.details_box)
            .child_size(ui::size_builder()
                .height_auto()
                .width_auto()
                .build())
            .child_align_self(ui::AlignSelf::Stretch)
            .child_flex_grow(1.0)

            .child_layout(&self.buttons_layout)
            .child_align_self(ui::AlignSelf::Stretch)

            .build(&self.root_layout)?;

        Ok(())
    }
}
