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

pub mod all_settings;
pub mod labels;
mod duckdb_setting;
mod config_error;
pub mod registry;

pub use all_settings::all_settings;
pub use config_error::ConfigError;
pub use registry::DsnType;
pub use registry::RegistryDsn;
pub use registry::RegistrySetting;
pub use duckdb_setting::DuckDbSetting;
