mod dns;
mod commands;
mod utils;

pub use dns::{DnsTestResult, UrlTestResult, BulkTestResult};
pub use commands::*;
pub use utils::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![test_dns_servers, cancel_dns_tests, get_current_session, test_url_with_dns_server, bulk_test_url_with_all_dns_servers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
