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
pub(super) struct AboutDialogLayout {
    root_layout: nwg::FlexboxLayout,
}

impl ui::Layout<AboutDialogControls> for AboutDialogLayout {
    fn build(&self, c: &AboutDialogControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Column)

            .child(&c.label)
            .child_size(ui::size_builder()
                .width_auto()
                .height_pt(50)
                .build())
            .child_flex_grow(1.0)

            .child(&c.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_align_self(ui::AlignSelf::FlexEnd)

            .build(&self.root_layout)?;

        Ok(())
    }
}
