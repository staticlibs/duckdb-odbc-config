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

use winreg::RegKey;
use winreg::enums;

pub const DRIVER_KEY_PATH: &str = "SOFTWARE\\ODBC\\ODBCINST.INI\\DuckDB Driver";
pub const ODBC_INI_SUBPATH: &str = "SOFTWARE\\ODBC\\ODBC.INI";
pub const DRIVER_SETTING_NAME: &str = "Driver";
pub const DATABASE_SETTING_NAME: &str = "database";
pub const DS_LISTING_SUBPATH: &str = "ODBC Data Sources";
pub const DRIVER_LISTING_LABEL: &str = "DuckDB Driver";

#[derive(Debug, Clone)]
pub struct RegistrySetting {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum DsnType {
    USER,
    SYSTEM,
}

#[derive(Debug, Clone)]
pub struct RegistryDsn {
    pub name: String,
    pub dsn_type: DsnType,
    pub settings: Vec<RegistrySetting>,
}

impl Default for RegistryDsn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            dsn_type: DsnType::USER,
            settings: vec!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Root {
    HKLM,
    HKCU,
}

pub fn duckdb_driver_path() -> Result<String, ConfigError> {
    let hklm = RegKey::predef(enums::HKEY_LOCAL_MACHINE);
    let driver_key = match hklm.open_subkey(DRIVER_KEY_PATH) {
        Ok(key) => key,
        Err(e) => return Err(ConfigError::from_string(format!(
            "DuckDB Driver not found in registry, path: 'HKLM\\{}', message: {}", DRIVER_KEY_PATH, e)))
    };
    match driver_key.get_value(DRIVER_SETTING_NAME) {
        Ok(path) => Ok(path),
        Err(e) => Err(ConfigError::from_string(format!(
            "Error reading the 'Driver' value from registry key: 'HKLM\\{}', message: {}", DRIVER_KEY_PATH, e)))
    }
}

pub fn list_subkeys(root: Root, path: &str) -> Result<Vec<String>, ConfigError> {
    let key = open_key(root, path, enums::KEY_READ)?;
    let res = key.enum_keys()
        .filter_map(|r| match r {
            Ok(r) => Some(r),
            Err(_) => None,
        })
        .collect();
    Ok(res)
}

pub fn list_values(root: Root, path: &str) -> Result<Vec<RegistrySetting>, ConfigError> {
    let key = open_key(root, path, enums::KEY_READ)?;
    let res = key.enum_values()
        .filter_map(|r| match r {
            Ok((name, rval)) => {
                let value = format!("{}", rval);
                let rs = RegistrySetting { name, value };
                Some(rs)
            },
            Err(_) => None,
        })
        .collect();
    Ok(res)
}

pub fn create_dsn(dsn_type: DsnType, name: &str,  database: &str) -> Result<(), ConfigError> {
    let root = match dsn_type {
        DsnType::USER => Root::HKCU,
        DsnType::SYSTEM => Root::HKLM,
    };
    let odbc_ini_key = open_key(root, ODBC_INI_SUBPATH, enums::KEY_READ)?;
    match odbc_ini_key.open_subkey(name) {
        Ok(_) => return Err(ConfigError::from_string(format!(
            "Data source already exist, name: {}", name))),
        Err(_) => {},
    }
    odbc_ini_key.create_subkey(name)?;
    let dsn_key = odbc_ini_key.open_subkey_with_flags(name, enums::KEY_SET_VALUE)?;
    let driver_path = duckdb_driver_path()?;
    dsn_key.set_value(DRIVER_SETTING_NAME, &driver_path)?;
    dsn_key.set_value(DATABASE_SETTING_NAME, &database.to_string())?;
    let listing_key = odbc_ini_key.open_subkey_with_flags(DS_LISTING_SUBPATH, enums::KEY_SET_VALUE)?;
    listing_key.set_value(name, &DRIVER_LISTING_LABEL)?;
    Ok(())
}

pub fn delete_dsn(dsn_type: DsnType, name: &str) -> Result<(), ConfigError> {
    let root = match dsn_type {
        DsnType::USER => Root::HKCU,
        DsnType::SYSTEM => Root::HKLM,
    };
    let odbc_ini_key = open_key(root, ODBC_INI_SUBPATH, enums::KEY_WRITE)?;
    odbc_ini_key.delete_subkey(name)?;
    let listing_key = odbc_ini_key.open_subkey_with_flags(DS_LISTING_SUBPATH, enums::KEY_SET_VALUE)?;
    listing_key.delete_value(name)?;
    Ok(())
}

pub fn set_dsn_value(dsn_type: DsnType, dsn_name: &str, st_name: &str, value: &str) -> Result<(), ConfigError>{
    let root = match dsn_type {
        DsnType::USER => Root::HKCU,
        DsnType::SYSTEM => Root::HKLM,
    };
    let dsn_path = format!("{}\\{}", ODBC_INI_SUBPATH, dsn_name);
    let dsn_key = open_key(root, &dsn_path, enums::KEY_SET_VALUE)?;
    dsn_key.set_value(st_name, &value.to_string())?;
    Ok(())
}

pub fn delete_dsn_value(dsn_type: DsnType, dsn_name: &str, st_name: &str) -> Result<(), ConfigError>{
    let root = match dsn_type {
        DsnType::USER => Root::HKCU,
        DsnType::SYSTEM => Root::HKLM,
    };
    let dsn_path = format!("{}\\{}", ODBC_INI_SUBPATH, dsn_name);
    let dsn_key = open_key(root, &dsn_path, enums::KEY_SET_VALUE)?;
    dsn_key.delete_value(st_name)?;
    Ok(())
}

fn open_key(root: Root, path: &str, perms: u32) -> Result<RegKey, ConfigError> {
    let root_key = match root {
        Root::HKLM => RegKey::predef(enums::HKEY_LOCAL_MACHINE),
        Root::HKCU => RegKey::predef(enums::HKEY_CURRENT_USER),
    };
    match root_key.open_subkey_with_flags(path, perms) {
        Ok(key) => Ok(key),
        Err(e) => Err(ConfigError::from_string(format!(
            "Cannot open registry key, path: '{:?}\\{}', message: {}", root, path, e)))
    }
}
