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
pub struct LoadDsnsDialog {
    pub(super) c: LoadDsnsDialogControls,

    loaded_dsns: LoadDsnsDialogResult,
    args: LoadDsnsDialogArgs,
    load_join_handle: ui::PopupJoinHandle<LoadDsnsResult>,
}

impl LoadDsnsDialog {
    pub fn on_load_complete(&mut self, _: nwg::EventData) {
        self.c.load_notice.receive();
        let res = self.load_join_handle.join();
        self.stop_progress_bar(res.success);
        if res.success {
            self.loaded_dsns = LoadDsnsDialogResult::new(res.dsns);
            self.close(nwg::EventData::NoData);
            return;
        }
        self.c.label.set_text("Load settings failed");
        self.c.details_box.set_text(&res.message);
        self.c.copy_clipboard_button.set_enabled(true);
        self.c.close_button.set_enabled(true);
    }

    pub fn copy_to_clipboard(&mut self, _: nwg::EventData) {
        let text = self.c.details_box.text();
        let _ = set_clipboard(formats::Unicode, &text);
    }

    pub fn stop_progress_bar(&self, success: bool) {
        self.c.progress_bar.set_marquee(false, 0);
        self.c.progress_bar.remove_flags(nwg::ProgressBarFlags::MARQUEE);
        self.c.progress_bar.set_pos(1);
        if !success {
            self.c.progress_bar.set_state(nwg::ProgressBarState::Error)
        }
    }

    fn load_dsns_from_registry() -> Result<Vec<RegistryDsn>, ConfigError> {
        let duckdb_driver_path = registry::duckdb_driver_path()?;
        let system_dsns = registry::list_subkeys(registry::Root::HKLM, registry::ODBC_INI_SUBPATH)?;
        let user_dsns = registry::list_subkeys(registry::Root::HKCU, registry::ODBC_INI_SUBPATH)?;
        let mut res: Vec <RegistryDsn> = vec!();
        for name in system_dsns {
            let dsn_subpath = format!("{}\\{}", registry::ODBC_INI_SUBPATH, name);
            let settings = registry::list_values(registry::Root::HKLM, &dsn_subpath)?;
            if settings.iter().any(|rs| "Driver" == rs.name && duckdb_driver_path == rs.value) {
                let dsn_type = DsnType::SYSTEM;
                res.push(RegistryDsn { name, dsn_type, settings })
            }
        }
        for name in user_dsns {
            let dsn_subpath = format!("{}\\{}", registry::ODBC_INI_SUBPATH, name);
            let settings = registry::list_values(registry::Root::HKCU, &dsn_subpath)?;
            if settings.iter().any(|rs| "Driver" == rs.name && duckdb_driver_path == rs.value) {
                let dsn_type = DsnType::USER;
                res.push(RegistryDsn { name, dsn_type, settings })
            }
        }
        Ok(res)
    }
}

impl ui::PopupDialog<LoadDsnsDialogArgs, LoadDsnsDialogResult> for LoadDsnsDialog {
    fn popup(args: LoadDsnsDialogArgs) -> ui::PopupJoinHandle<LoadDsnsDialogResult> {
        let join_handle = thread::spawn(move || {
            let data = Self {
                args,
                ..Default::default()
            };
            let mut dialog = Self::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            dialog.result()
        });
        ui::PopupJoinHandle::from(join_handle)
    }

    fn init(&mut self) {
        let sender = self.c.load_notice.sender();
        let join_handle = thread::spawn(move || {
            let start = Instant::now();
            let res = match LoadDsnsDialog::load_dsns_from_registry() {
                Ok(dsns) => LoadDsnsResult::success(dsns),
                Err(e) => LoadDsnsResult::failure(format!("{}", e))
            };
            let remaining = 1000 - start.elapsed().as_millis() as i64;
            if remaining > 0 {
                thread::sleep(Duration::from_millis(remaining as u64));
            }
            sender.send();
            res
        });
        self.load_join_handle = ui::PopupJoinHandle::from(join_handle);
    }

    fn result(&mut self) -> LoadDsnsDialogResult {
        self.loaded_dsns.clone()
    }

    fn close(&mut self, _: nwg::EventData) {
        self.args.notify_parent();
        self.c.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }

    fn on_resize(&mut self, _: nwg::EventData) {
        self.c.update_tab_order();
    }
}

