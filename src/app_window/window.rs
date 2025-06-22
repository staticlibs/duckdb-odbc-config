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

use std::env;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::process::Stdio;

use wildmatch::WildMatch;

use super::*;

#[derive(Default)]
pub struct AppWindow {
    pub(super) c: AppWindowControls,

    dsns: Vec<RegistryDsn>,
    settings: Vec<DuckDbSetting>,

    last_added_dsn: Option<String>,

    about_dialog_join_handle: ui::PopupJoinHandle<()>,
    load_settings_dialog_join_handle: ui::PopupJoinHandle<LoadDsnsDialogResult>,
    setting_dialog_join_handle: ui::PopupJoinHandle<SettingDialogResult>,
    add_dsn_dialog_join_handle: ui::PopupJoinHandle<AddDsnDialogResult>,
}

impl AppWindow {

    pub fn new() -> Self {
        Default::default()
    }

    pub(super) fn init(&mut self) {
        self.open_load_dialog(nwg::EventData::NoData);
    }

    pub(super) fn close(&mut self, _: nwg::EventData) {
        self.c.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }

    pub(super) fn open_about_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(false);
        let args = AboutDialogArgs::new(&self.c.about_notice);
        self.about_dialog_join_handle = AboutDialog::popup(args);
    }

    pub(super) fn await_about_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.about_notice.receive();
        let _ = self.about_dialog_join_handle.join();
        self.c.filter_input.set_enabled(true);
        self.c.conn_str_input.set_enabled(true);
    }

    pub(super) fn open_load_dialog(&mut self, _: nwg::EventData) {
        self.dsns.truncate(0);
        self.c.window.set_enabled(false);
        let args = LoadDsnsDialogArgs::new(&self.c.load_settings_notice);
        self.load_settings_dialog_join_handle = LoadDsnsDialog::popup(args);
    }

    pub(super) fn await_load_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.load_settings_notice.receive();
        let res = self.load_settings_dialog_join_handle.join();
        self.dsns = res.dsns;
        self.settings.truncate(0);
        self.c.filter_input.set_enabled(true);
        self.c.conn_str_input.set_enabled(true);
        self.reload_dsns_combo();
        if let Some(added) = &self.last_added_dsn {
            self.c.dsn_combo.set_selection_string(added);
            self.last_added_dsn = None;
        }
        self.on_dsn_changed(nwg::EventData::NoData);
    }
    
    fn non_engine_setting_description(name: &str) -> String {
        if registry::DATABASE_SETTING_NAME == name {
            registry::DATABASE_SETTING_DESCRIPTION.to_string()
        } else if registry::SESSION_INIT_SQL_FILE_SETTING_NAME == name{
            registry::SESSION_INIT_SQL_FILE_SETTING_DESCRIPTION.to_string()
        } else {
            "".to_string()
        }
    }

    pub(super) fn on_dsn_changed(&mut self, _: nwg::EventData) {
        self.settings = all_settings();
        self.c.conn_str_input.set_text("");
        if let Some(dname) = self.c.dsn_combo.selection_string() {
            if let Some(dsn) = self.dsns.iter().find(|d| d.name == dname) {
                self.c.conn_str_input.set_text(&format!("DSN={{{}}};", &dname));
                for rs in &dsn.settings {
                    if let Some(mut s) = self.settings.iter_mut().find(|s| s.name == rs.name) {
                        s.dsn_value = rs.value.to_string();
                    } else if "Driver" != rs.name {
                        self.settings.push(DuckDbSetting {
                            name: rs.name.to_string(),
                            dsn_value: rs.value.to_string(),
                            description: Self::non_engine_setting_description(&rs.name),
                            ..Default::default()
                        })
                    }
                }
            }
        }
        self.sort_settings(0, false);
        self.sort_settings(1, true);
        self.reload_settings_view();
    }

    pub(super) fn open_setting_dialog(&mut self, ed: nwg::EventData) {
        if let Some(name) = self.c.dsn_combo.selection_string() {
            if let Some(dsn) = self.dsns.iter().find(|d| d.name == name) {
                let row_idx = if let nwg::EventData::OnListViewItemIndex
                { row_index: row_idx, .. } = ed {
                    row_idx
                } else {
                    return;
                };
                let name = match self.c.settings_view.item(row_idx, 0, 1<<16) {
                    Some(item) => item.text,
                    None => return
                };
                let setting = match self.settings.iter().find(|s| s.name == name) {
                    Some(st) => st.clone(),
                    None => return
                };
                self.c.window.set_enabled(false);
                let args = SettingDialogArgs::new(&self.c.setting_notice, dsn.clone(), setting);
                self.setting_dialog_join_handle = SettingDialog::popup(args);
            }
        }
    }

    pub(super) fn await_setting_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.setting_notice.receive();
        let res = self.setting_dialog_join_handle.join();
        self.c.filter_input.set_enabled(true);
        self.c.conn_str_input.set_enabled(true);
        if res.success {
            self.open_load_dialog(nwg::EventData::NoData);
        }
    }

    pub(super) fn open_add_dsn_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(false);
        let args = AddDsnDialogArgs::new(&self.c.add_dsn_notice);
        self.add_dsn_dialog_join_handle = AddDsnDialog::popup(args);
    }

    pub(super) fn await_add_dsn_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.add_dsn_notice.receive();
        let res = self.add_dsn_dialog_join_handle.join();
        self.c.filter_input.set_enabled(true);
        self.c.conn_str_input.set_enabled(true);
        self.last_added_dsn = res.added_dsn;
        if self.last_added_dsn.is_some() {
            self.open_load_dialog(nwg::EventData::NoData)
        }
    }

    pub(super) fn on_delete_dsn_button(&mut self, _: nwg::EventData) {
        if let Some(name) = self.c.dsn_combo.selection_string() {
            if let Some(dsn) = self.dsns.iter().find(|d| d.name == name) {
                let confirmed = ui::message_box_warning_yn(&format!(
                    "Data source: '{}' will be removed from registry, would you like to proceed?", dsn.name));
                if !confirmed {
                    return;
                }
                match registry::delete_dsn(dsn.dsn_type.clone(), &dsn.name) {
                    Ok(_) => self.open_load_dialog(nwg::EventData::NoData),
                    Err(e) => ui::message_box_error(&format!(
                        "Error removing DNS from registry, type: {:?}, name: {}, message: {}", &dsn.dsn_type, &dsn.name, e))
                }
            }
        }
    }

    pub(super) fn open_website(&mut self, _: nwg::EventData) {
        let create_no_window: u32 = 0x08000000;
        let _ = Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("https://duckdb.org/docs/clients/odbc/windows")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(create_no_window)
            .status();
    }

    pub(super) fn on_settings_view_sort(&mut self, ed: nwg::EventData) {
        let col_idx = if let nwg::EventData::OnListViewItemIndex
                { column_index: col_idx, .. } = ed {
            col_idx
        } else {
           return;
        };
        if col_idx > 2 {
            return;
        }
        let old_arrow = self.c.settings_view
            .column_sort_arrow(col_idx)
            .expect("Sort not initialized");
        let arrow = match old_arrow {
            nwg::ListViewColumnSortArrow::Up => nwg::ListViewColumnSortArrow::Down,
            nwg::ListViewColumnSortArrow::Down => nwg::ListViewColumnSortArrow::Up
        };
        let desc = match arrow {
            nwg::ListViewColumnSortArrow::Up => true,
            nwg::ListViewColumnSortArrow::Down => false
        };
        self.sort_settings(col_idx, desc);
        self.c.settings_view.set_column_sort_arrow(col_idx, Some(arrow));
        self.reload_settings_view();
    }

    pub(super) fn on_filter_button(&mut self, _: nwg::EventData) {
        self.reload_settings_view()
    }

    pub(super) fn on_copy_conn_str_button(&mut self, _: nwg::EventData) {
        use clipboard_win::formats;
        use clipboard_win::set_clipboard;

        let text = self.c.conn_str_input.text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub(super) fn on_resize(&mut self, _: nwg::EventData) {
        self.c.update_tab_order();
    }

    #[allow(dead_code)]
    fn get_cmd_args() -> Vec<String> {
        let mut res = vec!();
        for aos in env::args_os() {
            match aos.into_string() {
                Ok(st) => res.push(st),
                Err(_) => {/* ignore */}
            }
        };
        res
    }

    fn setting_matches_filters(&self, name: &str) -> bool {
        let filter = self.c.filter_input.text();
        if 0 == filter.len() {
            return true;
        }
        if name.starts_with(&filter) {
            return true;
        }
        WildMatch::new(&filter).matches(name)
    }

    fn reload_dsns_combo(&self) {
        let dc = &self.c.dsn_combo;
        let sel_opt = dc.selection_string();
        while dc.len() > 0 {
            dc.remove(0);
        }
        for dsn in &self.dsns {
            dc.push(dsn.name.clone())
        }
        if self.dsns.len() > 0 {
            if let Some(sel) = sel_opt {
                let sel_set = dc.set_selection_string(&sel);
                if sel_set.is_none() {
                    dc.set_selection(Some(0));
                }
            } else {
                dc.set_selection(Some(0));
            }
        }
    }

    fn reload_settings_view(&self) {
        let sv = &self.c.settings_view;
        sv.set_redraw(false);
        loop {
            let removed = sv.remove_item(0);
            if !removed {
                break;
            }
        };
        let mut idx = 0 as i32;
        for rec in &self.settings {
            if self.setting_matches_filters(&rec.name) {
                sv.insert_item(nwg::InsertListViewItem {
                    index: Some(idx as i32),
                    column_index: 0,
                    text: Some(rec.name.clone()),
                    image: None
                });
                sv.insert_item(nwg::InsertListViewItem {
                    index: Some(idx as i32),
                    column_index: 1,
                    text: Some(rec.dsn_value.clone()),
                    image: None
                });
                sv.insert_item(nwg::InsertListViewItem {
                    index: Some(idx as i32),
                    column_index: 2,
                    text: Some(rec.default_value.clone()),
                    image: None
                });
                sv.insert_item(nwg::InsertListViewItem {
                    index: Some(idx as i32),
                    column_index: 3,
                    text: Some(rec.description.clone()),
                    image: None
                });
                idx += 1;
            }
        }
        sv.set_redraw(true);
    }

    fn sort_settings(&mut self, col_idx: usize, desc: bool) {
        if 0 == col_idx {
            self.settings.sort_by(|a, b| {
                if desc {
                    b.name.to_lowercase().cmp(&a.name.to_lowercase())
                } else {
                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                }
            });
        } else if 1 == col_idx {
            self.settings.sort_by(|a, b| {
                if desc {
                    b.dsn_value.to_lowercase().cmp(&a.dsn_value.to_lowercase())
                } else {
                    a.dsn_value.to_lowercase().cmp(&b.dsn_value.to_lowercase())
                }
            });
        } else if 2 == col_idx {
            self.settings.sort_by(|a, b| {
                if desc {
                    b.default_value.to_lowercase().cmp(&a.default_value.to_lowercase())
                } else {
                    a.default_value.to_lowercase().cmp(&b.default_value.to_lowercase())
                }
            });
        }
    }
}
