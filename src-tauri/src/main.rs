#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod image_fixer;
use image_fixer::{scan_directory, select_directory, demonstrate_emit_events};

fn main() {
    // 重定向标准错误到控制台
    #[cfg(debug_assertions)]
    {
        use std::io::Write;

        // 在调试模式下，确保标准错误输出到控制台
        let stderr = std::io::stderr();
        let mut handle = stderr.lock();

        // 写入一条测试消息来验证错误输出
        let _ = writeln!(handle, "File Time Fixer 应用程序启动...");
        let _ = writeln!(handle, "Rust 标准错误已重定向到控制台");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![scan_directory, select_directory, demonstrate_emit_events])
        .run(tauri::generate_context!())
        .expect("运行Tauri应用程序时出错");
}
