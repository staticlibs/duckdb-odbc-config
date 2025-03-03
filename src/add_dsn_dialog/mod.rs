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

mod args;
mod controls;
mod dialog;
mod events;
mod layout;
mod nui;
mod result;

use std::thread;

use nwg::NativeUi;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::Events;
use ui::Layout;
use ui::PopupArgs;
use ui::PopupDialog;

pub use args::AddDsnDialogArgs;
use common::registry;
use controls::AddDsnDialogControls;
pub use dialog::AddDsnDialog;
use events::AddDsnDialogEvents;
use layout::AddDsnDialogLayout;
pub use result::AddDsnDialogResult;
