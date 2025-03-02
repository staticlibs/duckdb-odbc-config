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

use std::cell::RefCell;
use std::rc::Rc;

use super::*;

pub(super) struct SettingChangeDialogNui {
    inner: Rc<RefCell<SettingChangeDialog>>,
    inner_events: Rc<SettingChangeDialogEvents>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl SettingChangeDialogNui {
    pub(super) fn result(&mut self) -> SettingChangeDialogResult {
        self.inner.borrow_mut().result()
    }
}

impl nwg::NativeUi<SettingChangeDialogNui> for SettingChangeDialog {
    fn build_ui(mut dialog: SettingChangeDialog) -> Result<SettingChangeDialogNui, nwg::NwgError> {
        let mut events: SettingChangeDialogEvents = Default::default();
        dialog.c.build()?;
        events.build(&dialog.c)?;
        dialog.init();
        dialog.c.update_tab_order();

        let window_handle = dialog.c.window.handle.clone();

        let wrapper = SettingChangeDialogNui {
            inner:  Rc::new(RefCell::new(dialog)),
            inner_events: Rc::new(events),
            default_handler: Default::default(),
        };

        let dialog_ref = Rc::downgrade(&wrapper.inner);
        let events_ref = Rc::downgrade(&wrapper.inner_events);
        let handle_events = move |evt, evt_data, handle| {
            if let Some(evt_dialog_ref) = dialog_ref.upgrade() {
                if let Some(evt_events_ref) = events_ref.upgrade() {
                    for eh in evt_events_ref.events.iter() {
                        if handle == eh.control_handle && evt == eh.event {
                            let mut evt_dialog = evt_dialog_ref.borrow_mut();
                            (eh.handler)(&mut evt_dialog, evt_data);
                            break;
                        }
                    }
                }
            }
        };

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&window_handle, handle_events));

        return Ok(wrapper);
    }
}

impl Drop for SettingChangeDialogNui {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}
