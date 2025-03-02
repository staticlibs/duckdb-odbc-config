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

mod controls;
mod events;
mod layout;
mod nui;
mod window;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::Events;
use ui::Layout;
use ui::PopupDialog;

use about_dialog::AboutDialog;
use about_dialog::AboutDialogArgs;
use add_dsn_dialog::AddDsnDialog;
use add_dsn_dialog::AddDsnDialogArgs;
use add_dsn_dialog::AddDsnDialogResult;
use common::all_settings;
use common::registry;
use common::DuckDbSetting;
use common::RegistryDsn;
use load_dsns_dialog::LoadDsnsDialog;
use load_dsns_dialog::LoadDsnsDialogArgs;
use load_dsns_dialog::LoadDsnsDialogResult;
use setting_dialog::SettingDialog;
use setting_dialog::SettingDialogArgs;
use setting_dialog::SettingDialogResult;

pub(self) use controls::AppWindowControls;
pub(self) use events::AppWindowEvents;
use layout::AppWindowLayout;
pub use window::AppWindow;
