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

#[derive(Default, Clone)]
pub struct LoadDsnsDialogResult {
    pub dsns: Vec<RegistryDsn>
}

impl LoadDsnsDialogResult {
    pub fn new(dsns: Vec<RegistryDsn>) -> Self {
        Self { dsns }
    }
}

#[derive(Default)]
pub struct LoadDsnsResult {
    pub success: bool,
    pub message: String,
    pub dsns: Vec<RegistryDsn>,
}

impl LoadDsnsResult {
    pub fn success(dsns: Vec<RegistryDsn>) -> Self {
        Self {
            success: true,
            message: String::new(),
            dsns,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            dsns: Vec::new(),
        }
    }
}
