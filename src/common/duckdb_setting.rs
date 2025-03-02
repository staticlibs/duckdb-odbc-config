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

#[derive(Default, Debug, Clone)]
pub struct DuckDbSetting {
    pub name: String,
    pub dsn_value: String,
    pub default_value: String,
    pub description: String,
    pub input_type: String,
    pub scope: String,
}

impl DuckDbSetting {
    pub fn new(name: &str, value: &str, input_type: &str, scope: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            dsn_value: "".to_string(),
            default_value: value.to_string(),
            description: description.to_string(),
            input_type: input_type.to_string(),
            scope: scope.to_string(),
        }
    }
}