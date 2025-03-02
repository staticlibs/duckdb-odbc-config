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
pub struct LoadDsnsDialogArgs {
    notice_sender:  ui::SyncNoticeSender,
}

impl LoadDsnsDialogArgs {
    pub fn new(notice: &ui::SyncNotice) -> Self {
        Self {
            notice_sender: notice.sender(),
        }
    }
}

impl ui::PopupArgs for LoadDsnsDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
