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

#[derive(Default)]
pub(super) struct ChangeResult {
    pub(super) success: bool,
    pub(super) effective_value: String,
    pub(super) error: String,
}

impl ChangeResult {
    pub(super) fn success(effective_value: String) -> Self {
        Self {
            success: true,
            effective_value,
            error: String::new()
        }
    }

    pub(super) fn failure(error: String) -> Self {
        Self {
            success: false,
            effective_value: String::new(),
            error
        }
    }
}

#[derive(Default, Clone)]
pub struct SettingChangeDialogResult {
    pub success: bool,
    pub effective_value: String,
}

impl SettingChangeDialogResult {
    pub(super) fn success(effective_value: String) -> Self {
        Self {
            success: true,
            effective_value
        }
    }

    pub(super) fn failure() -> Self {
        Self {
            success: false,
            effective_value: String::new()
        }
    }
}
