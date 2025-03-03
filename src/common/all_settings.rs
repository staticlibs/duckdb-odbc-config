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

// Generated by: python resources\duckdb_settings.py > src\commonll_settings.rs

pub fn all_settings() -> Vec<DuckDbSetting> {
    vec!(
DuckDbSetting::new("access_mode", r#"automatic"#, "VARCHAR", "GLOBAL", r#"Access mode of the database (AUTOMATIC, READ_ONLY or READ_WRITE)"#),
DuckDbSetting::new("allocator_background_threads", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether to enable the allocator background thread."#),
DuckDbSetting::new("allocator_bulk_deallocation_flush_threshold", r#"512.0 MiB"#, "VARCHAR", "GLOBAL", r#"If a bulk deallocation larger than this occurs, flush outstanding allocations."#),
DuckDbSetting::new("allocator_flush_threshold", r#"128.0 MiB"#, "VARCHAR", "GLOBAL", r#"Peak allocation threshold at which to flush the allocator after completing a task."#),
DuckDbSetting::new("allow_community_extensions", r#"true"#, "BOOLEAN", "GLOBAL", r#"Allow to load community built extensions"#),
DuckDbSetting::new("allow_extensions_metadata_mismatch", r#"false"#, "BOOLEAN", "GLOBAL", r#"Allow to load extensions with not compatible metadata"#),
DuckDbSetting::new("allow_persistent_secrets", r#"true"#, "BOOLEAN", "GLOBAL", r#"Allow the creation of persistent secrets, that are stored and loaded on restarts"#),
DuckDbSetting::new("allow_unredacted_secrets", r#"false"#, "BOOLEAN", "GLOBAL", r#"Allow printing unredacted secrets"#),
DuckDbSetting::new("allow_unsigned_extensions", r#"false"#, "BOOLEAN", "GLOBAL", r#"Allow to load extensions with invalid or missing signatures"#),
DuckDbSetting::new("allowed_directories", r#"[]"#, "VARCHAR[]", "GLOBAL", r#"List of directories/prefixes that are ALWAYS allowed to be queried - even when enable_external_access is false"#),
DuckDbSetting::new("allowed_paths", r#"[]"#, "VARCHAR[]", "GLOBAL", r#"List of files that are ALWAYS allowed to be queried - even when enable_external_access is false"#),
DuckDbSetting::new("arrow_large_buffer_size", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether Arrow buffers for strings, blobs, uuids and bits should be exported using large buffers"#),
DuckDbSetting::new("arrow_lossless_conversion", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whenever a DuckDB type does not have a clear native or canonical extension match in Arrow, export the types with a duckdb.type_name extension name."#),
DuckDbSetting::new("arrow_output_list_view", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether export to Arrow format should use ListView as the physical layout for LIST columns"#),
DuckDbSetting::new("asof_loop_join_threshold", r#"64"#, "UBIGINT", "LOCAL", r#"The maximum number of rows we need on the left side of an ASOF join to use a nested loop join"#),
DuckDbSetting::new("autoinstall_extension_repository", r#""#, "VARCHAR", "GLOBAL", r#"Overrides the custom endpoint for extension installation on autoloading"#),
DuckDbSetting::new("autoinstall_known_extensions", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether known extensions are allowed to be automatically installed when a query depends on them"#),
DuckDbSetting::new("autoload_known_extensions", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether known extensions are allowed to be automatically loaded when a query depends on them"#),
DuckDbSetting::new("binary_as_string", r#""#, "BOOLEAN", "GLOBAL", r#"In Parquet files, interpret binary data as a string."#),
DuckDbSetting::new("Calendar", r#"gregorian"#, "VARCHAR", "GLOBAL", r#"The current calendar"#),
DuckDbSetting::new("catalog_error_max_schemas", r#"100"#, "UBIGINT", "GLOBAL", r#"The maximum number of schemas the system will scan for "did you mean..." style errors in the catalog"#),
DuckDbSetting::new("checkpoint_threshold", r#"16.0 MiB"#, "VARCHAR", "GLOBAL", r#"The WAL size threshold at which to automatically trigger a checkpoint (e.g. 1GB)"#),
DuckDbSetting::new("custom_extension_repository", r#""#, "VARCHAR", "GLOBAL", r#"Overrides the custom endpoint for remote extension installation"#),
DuckDbSetting::new("custom_profiling_settings", r#"{"QUERY_NAME": "true", "BLOCKED_THREAD_TIME": "true", "CPU_TIME": "true", "EXTRA_INFO": "true", "CUMULATIVE_CARDINALITY": "true", "OPERATOR_TYPE": "true", "OPERATOR_NAME": "true", "OPERATOR_CARDINALI"#, "VARCHAR", "LOCAL", r#"Accepts a JSON enabling custom metrics"#),
DuckDbSetting::new("custom_user_agent", r#""#, "VARCHAR", "GLOBAL", r#"Metadata from DuckDB callers"#),
DuckDbSetting::new("debug_asof_iejoin", r#"false"#, "BOOLEAN", "LOCAL", r#"DEBUG SETTING: force use of IEJoin to implement AsOf joins"#),
DuckDbSetting::new("debug_checkpoint_abort", r#"none"#, "VARCHAR", "GLOBAL", r#"DEBUG SETTING: trigger an abort while checkpointing for testing purposes"#),
DuckDbSetting::new("debug_force_external", r#"false"#, "BOOLEAN", "LOCAL", r#"DEBUG SETTING: force out-of-core computation for operators that support it, used for testing"#),
DuckDbSetting::new("debug_force_no_cross_product", r#"false"#, "BOOLEAN", "LOCAL", r#"DEBUG SETTING: Force disable cross product generation when hyper graph isn't connected, used for testing"#),
DuckDbSetting::new("debug_skip_checkpoint_on_commit", r#"false"#, "BOOLEAN", "GLOBAL", r#"DEBUG SETTING: skip checkpointing on commit"#),
DuckDbSetting::new("debug_window_mode", r#"window"#, "VARCHAR", "GLOBAL", r#"DEBUG SETTING: switch window mode to use"#),
DuckDbSetting::new("default_block_size", r#"262144"#, "UBIGINT", "GLOBAL", r#"The default block size for new duckdb database files (new as-in, they do not yet exist)."#),
DuckDbSetting::new("default_collation", r#""#, "VARCHAR", "GLOBAL", r#"The collation setting used when none is specified"#),
DuckDbSetting::new("default_null_order", r#"nulls_last"#, "VARCHAR", "GLOBAL", r#"NULL ordering used when none is specified (NULLS_FIRST or NULLS_LAST)"#),
DuckDbSetting::new("default_order", r#"asc"#, "VARCHAR", "GLOBAL", r#"The order type used when none is specified (ASC or DESC)"#),
DuckDbSetting::new("default_secret_storage", r#"local_file"#, "VARCHAR", "GLOBAL", r#"Allows switching the default storage for secrets"#),
DuckDbSetting::new("disable_parquet_prefetching", r#"false"#, "BOOLEAN", "GLOBAL", r#"Disable the prefetching mechanism in Parquet"#),
DuckDbSetting::new("disabled_compression_methods", r#""#, "VARCHAR", "GLOBAL", r#"Disable a specific set of compression methods (comma separated)"#),
DuckDbSetting::new("disabled_filesystems", r#""#, "VARCHAR", "GLOBAL", r#"Disable specific file systems preventing access (e.g. LocalFileSystem)"#),
DuckDbSetting::new("disabled_log_types", r#""#, "VARCHAR", "GLOBAL", r#"Sets the list of disabled loggers"#),
DuckDbSetting::new("disabled_optimizers", r#""#, "VARCHAR", "GLOBAL", r#"DEBUG SETTING: disable a specific set of optimizers (comma separated)"#),
DuckDbSetting::new("duckdb_api", r#"odbc"#, "VARCHAR", "GLOBAL", r#"DuckDB API surface"#),
DuckDbSetting::new("dynamic_or_filter_threshold", r#"50"#, "UBIGINT", "LOCAL", r#"The maximum amount of OR filters we generate dynamically from a hash join"#),
DuckDbSetting::new("enable_external_access", r#"true"#, "BOOLEAN", "GLOBAL", r#"Allow the database to access external state (through e.g. loading/installing modules, COPY TO/FROM, CSV readers, pandas replacement scans, etc)"#),
DuckDbSetting::new("enable_fsst_vectors", r#"false"#, "BOOLEAN", "GLOBAL", r#"Allow scans on FSST compressed segments to emit compressed vectors to utilize late decompression"#),
DuckDbSetting::new("enable_geoparquet_conversion", r#"true"#, "BOOLEAN", "GLOBAL", r#"Attempt to decode/encode geometry data in/as GeoParquet files if the spatial extension is present."#),
DuckDbSetting::new("enable_http_logging", r#"false"#, "BOOLEAN", "LOCAL", r#"Enables HTTP logging"#),
DuckDbSetting::new("enable_http_metadata_cache", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether or not the global http metadata is used to cache HTTP metadata"#),
DuckDbSetting::new("enable_logging", r#"0"#, "BOOLEAN", "GLOBAL", r#"Enables the logger"#),
DuckDbSetting::new("enable_macro_dependencies", r#"false"#, "BOOLEAN", "GLOBAL", r#"Enable created MACROs to create dependencies on the referenced objects (such as tables)"#),
DuckDbSetting::new("enable_object_cache", r#"NULL"#, "BOOLEAN", "GLOBAL", r#"[PLACEHOLDER] Legacy setting - does nothing"#),
DuckDbSetting::new("enable_profiling", r#"NULL"#, "VARCHAR", "LOCAL", r#"Enables profiling, and sets the output format (JSON, QUERY_TREE, QUERY_TREE_OPTIMIZER)"#),
DuckDbSetting::new("enable_progress_bar", r#"false"#, "BOOLEAN", "LOCAL", r#"Enables the progress bar, printing progress to the terminal for long queries"#),
DuckDbSetting::new("enable_progress_bar_print", r#"true"#, "BOOLEAN", "LOCAL", r#"Controls the printing of the progress bar, when 'enable_progress_bar' is true"#),
DuckDbSetting::new("enable_view_dependencies", r#"false"#, "BOOLEAN", "GLOBAL", r#"Enable created VIEWs to create dependencies on the referenced objects (such as tables)"#),
DuckDbSetting::new("enabled_log_types", r#""#, "VARCHAR", "GLOBAL", r#"Sets the list of enabled loggers"#),
DuckDbSetting::new("errors_as_json", r#"false"#, "BOOLEAN", "LOCAL", r#"Output error messages as structured JSON instead of as a raw string"#),
DuckDbSetting::new("explain_output", r#"physical_only"#, "VARCHAR", "LOCAL", r#"Output of EXPLAIN statements (ALL, OPTIMIZED_ONLY, PHYSICAL_ONLY)"#),
DuckDbSetting::new("extension_directory", r#""#, "VARCHAR", "GLOBAL", r#"Set the directory to store extensions in"#),
DuckDbSetting::new("external_threads", r#"1"#, "UBIGINT", "GLOBAL", r#"The number of external threads that work on DuckDB tasks."#),
DuckDbSetting::new("file_search_path", r#""#, "VARCHAR", "LOCAL", r#"A comma separated list of directories to search for input files"#),
DuckDbSetting::new("force_bitpacking_mode", r#"auto"#, "VARCHAR", "GLOBAL", r#"DEBUG SETTING: forces a specific bitpacking mode"#),
DuckDbSetting::new("force_compression", r#"Auto"#, "VARCHAR", "GLOBAL", r#"DEBUG SETTING: forces a specific compression method to be used"#),
DuckDbSetting::new("home_directory", r#""#, "VARCHAR", "LOCAL", r#"Sets the home directory used by the system"#),
DuckDbSetting::new("http_logging_output", r#""#, "VARCHAR", "LOCAL", r#"The file to which HTTP logging output should be saved, or empty to print to the terminal"#),
DuckDbSetting::new("http_proxy", r#""#, "VARCHAR", "GLOBAL", r#"HTTP proxy host"#),
DuckDbSetting::new("http_proxy_password", r#""#, "VARCHAR", "GLOBAL", r#"Password for HTTP proxy"#),
DuckDbSetting::new("http_proxy_username", r#""#, "VARCHAR", "GLOBAL", r#"Username for HTTP proxy"#),
DuckDbSetting::new("ieee_floating_point_ops", r#"true"#, "BOOLEAN", "LOCAL", r#"Use IEE754-compliant floating point operations (returning NAN instead of errors/NULL)."#),
DuckDbSetting::new("immediate_transaction_mode", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether transactions should be started lazily when needed, or immediately when BEGIN TRANSACTION is called"#),
DuckDbSetting::new("index_scan_max_count", r#"2048"#, "UBIGINT", "GLOBAL", r#"The maximum index scan count sets a threshold for index scans. If fewer than MAX(index_scan_max_count, index_scan_percentage * total_row_count) rows match, we perform an index scan instead of a table"#),
DuckDbSetting::new("index_scan_percentage", r#"0.001"#, "DOUBLE", "GLOBAL", r#"The index scan percentage sets a threshold for index scans. If fewer than MAX(index_scan_max_count, index_scan_percentage * total_row_count) rows match, we perform an index scan instead of a table sc"#),
DuckDbSetting::new("integer_division", r#"false"#, "BOOLEAN", "LOCAL", r#"Whether or not the / operator defaults to integer division, or to floating point division"#),
DuckDbSetting::new("late_materialization_max_rows", r#"50"#, "UBIGINT", "LOCAL", r#"The maximum amount of rows in the LIMIT/SAMPLE for which we trigger late materialization"#),
DuckDbSetting::new("lock_configuration", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether or not the configuration can be altered"#),
DuckDbSetting::new("log_query_path", r#"NULL"#, "VARCHAR", "LOCAL", r#"Specifies the path to which queries should be logged (default: NULL, queries are not logged)"#),
DuckDbSetting::new("logging_level", r#"INFO"#, "VARCHAR", "GLOBAL", r#"The log level which will be recorded in the log"#),
DuckDbSetting::new("logging_mode", r#"LEVEL_ONLY"#, "VARCHAR", "GLOBAL", r#"Enables the logger"#),
DuckDbSetting::new("logging_storage", r#"memory"#, "VARCHAR", "GLOBAL", r#"Set the logging storage (memory/stdout/file)"#),
DuckDbSetting::new("max_expression_depth", r#"1000"#, "UBIGINT", "LOCAL", r#"The maximum expression depth limit in the parser. WARNING: increasing this setting and using very deep expressions might lead to stack overflow errors."#),
DuckDbSetting::new("max_memory", r#"6.3 GiB"#, "VARCHAR", "GLOBAL", r#"The maximum memory of the system (e.g. 1GB)"#),
DuckDbSetting::new("max_temp_directory_size", r#"90% of available disk space"#, "VARCHAR", "GLOBAL", r#"The maximum amount of data stored inside the 'temp_directory' (when set) (e.g. 1GB)"#),
DuckDbSetting::new("max_vacuum_tasks", r#"100"#, "UBIGINT", "GLOBAL", r#"The maximum vacuum tasks to schedule during a checkpoint."#),
DuckDbSetting::new("memory_limit", r#"6.3 GiB"#, "VARCHAR", "GLOBAL", r#"The maximum memory of the system (e.g. 1GB)"#),
DuckDbSetting::new("merge_join_threshold", r#"1000"#, "UBIGINT", "LOCAL", r#"The number of rows we need on either table to choose a merge join"#),
DuckDbSetting::new("nested_loop_join_threshold", r#"5"#, "UBIGINT", "LOCAL", r#"The number of rows we need on either table to choose a nested loop join"#),
DuckDbSetting::new("null_order", r#"nulls_last"#, "VARCHAR", "GLOBAL", r#"NULL ordering used when none is specified (NULLS_FIRST or NULLS_LAST)"#),
DuckDbSetting::new("old_implicit_casting", r#"false"#, "BOOLEAN", "GLOBAL", r#"Allow implicit casting to/from VARCHAR"#),
DuckDbSetting::new("order_by_non_integer_literal", r#"false"#, "BOOLEAN", "LOCAL", r#"Allow ordering by non-integer literals - ordering by such literals has no effect."#),
DuckDbSetting::new("ordered_aggregate_threshold", r#"262144"#, "UBIGINT", "LOCAL", r#"The number of rows to accumulate before sorting, used for tuning"#),
DuckDbSetting::new("parquet_metadata_cache", r#"false"#, "BOOLEAN", "GLOBAL", r#"Cache Parquet metadata - useful when reading the same files multiple times"#),
DuckDbSetting::new("partitioned_write_flush_threshold", r#"524288"#, "UBIGINT", "LOCAL", r#"The threshold in number of rows after which we flush a thread state when writing using PARTITION_BY"#),
DuckDbSetting::new("partitioned_write_max_open_files", r#"100"#, "UBIGINT", "LOCAL", r#"The maximum amount of files the system can keep open before flushing to disk when writing using PARTITION_BY"#),
DuckDbSetting::new("password", r#"NULL"#, "VARCHAR", "GLOBAL", r#"The password to use. Ignored for legacy compatibility."#),
DuckDbSetting::new("perfect_ht_threshold", r#"12"#, "UBIGINT", "LOCAL", r#"Threshold in bytes for when to use a perfect hash table"#),
DuckDbSetting::new("pivot_filter_threshold", r#"20"#, "UBIGINT", "LOCAL", r#"The threshold to switch from using filtered aggregates to LIST with a dedicated pivot operator"#),
DuckDbSetting::new("pivot_limit", r#"100000"#, "UBIGINT", "LOCAL", r#"The maximum number of pivot columns in a pivot statement"#),
DuckDbSetting::new("prefer_range_joins", r#"false"#, "BOOLEAN", "LOCAL", r#"Force use of range joins with mixed predicates"#),
DuckDbSetting::new("prefetch_all_parquet_files", r#"false"#, "BOOLEAN", "GLOBAL", r#"Use the prefetching mechanism for all types of parquet files"#),
DuckDbSetting::new("preserve_identifier_case", r#"true"#, "BOOLEAN", "LOCAL", r#"Whether or not to preserve the identifier case, instead of always lowercasing all non-quoted identifiers"#),
DuckDbSetting::new("preserve_insertion_order", r#"true"#, "BOOLEAN", "GLOBAL", r#"Whether or not to preserve insertion order. If set to false the system is allowed to re-order any results that do not contain ORDER BY clauses."#),
DuckDbSetting::new("produce_arrow_string_view", r#"false"#, "BOOLEAN", "GLOBAL", r#"Whether strings should be produced by DuckDB in Utf8View format instead of Utf8"#),
DuckDbSetting::new("profile_output", r#""#, "VARCHAR", "LOCAL", r#"The file to which profile output should be saved, or empty to print to the terminal"#),
DuckDbSetting::new("profiling_mode", r#"NULL"#, "VARCHAR", "LOCAL", r#"The profiling mode (STANDARD or DETAILED)"#),
DuckDbSetting::new("profiling_output", r#""#, "VARCHAR", "LOCAL", r#"The file to which profile output should be saved, or empty to print to the terminal"#),
DuckDbSetting::new("progress_bar_time", r#"2000"#, "BIGINT", "LOCAL", r#"Sets the time (in milliseconds) how long a query needs to take before we start printing a progress bar"#),
DuckDbSetting::new("scalar_subquery_error_on_multiple_rows", r#"true"#, "BOOLEAN", "LOCAL", r#"When a scalar subquery returns multiple rows - return a random row instead of returning an error."#),
DuckDbSetting::new("schema", r#"main"#, "VARCHAR", "LOCAL", r#"Sets the default search schema. Equivalent to setting search_path to a single value."#),
DuckDbSetting::new("search_path", r#""#, "VARCHAR", "LOCAL", r#"Sets the default catalog search path as a comma-separated list of values"#),
DuckDbSetting::new("secret_directory", r#"C:\Users\alexk\.duckdb\stored_secrets"#, "VARCHAR", "GLOBAL", r#"Set the directory to which persistent secrets are stored"#),
DuckDbSetting::new("storage_compatibility_version", r#"v0.10.2"#, "VARCHAR", "GLOBAL", r#"Serialize on checkpoint with compatibility for a given duckdb version"#),
DuckDbSetting::new("streaming_buffer_size", r#"976.5 KiB"#, "VARCHAR", "LOCAL", r#"The maximum memory to buffer between fetching from a streaming result (e.g. 1GB)"#),
DuckDbSetting::new("temp_directory", r#".tmp"#, "VARCHAR", "GLOBAL", r#"Set the directory to which to write temp files"#),
DuckDbSetting::new("threads", r#"4"#, "BIGINT", "GLOBAL", r#"The number of total threads used by the system."#),
DuckDbSetting::new("TimeZone", r#"Europe/Dublin"#, "VARCHAR", "GLOBAL", r#"The current time zone"#),
DuckDbSetting::new("user", r#"NULL"#, "VARCHAR", "GLOBAL", r#"The username to use. Ignored for legacy compatibility."#),
DuckDbSetting::new("username", r#"NULL"#, "VARCHAR", "GLOBAL", r#"The username to use. Ignored for legacy compatibility."#),
DuckDbSetting::new("wal_autocheckpoint", r#"16.0 MiB"#, "VARCHAR", "GLOBAL", r#"The WAL size threshold at which to automatically trigger a checkpoint (e.g. 1GB)"#),
DuckDbSetting::new("worker_threads", r#"4"#, "BIGINT", "GLOBAL", r#"The number of total threads used by the system."#),
DuckDbSetting::new("zstd_min_string_length", r#"4096"#, "UBIGINT", "GLOBAL", r#"The (average) length at which to enable ZSTD compression, defaults to 4096"#),
)
}
