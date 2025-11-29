#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// 导入共享函数
use file_time_fixer_lib::build_and_run_app;
// use lib::build_and_run_app;

// 导入image_fixer模块（虽然在lib.rs中已经导入，但为了明确依赖关系，这里再次导入）
mod image_fixer;

fn main() {
    // 简化的应用启动日志，不依赖额外的日志库
    println!("File Time Fixer 应用程序启动");
    
    // 桌面平台重定向标准错误到控制台
    #[cfg(debug_assertions)]
    {
        use std::io::Write;

        // 在调试模式下，确保标准错误输出到控制台
        let stderr = std::io::stderr();
        let mut handle = stderr.lock();

        // 写入一条测试消息来验证错误输出
        let _ = writeln!(handle, "Rust 标准错误已重定向到控制台");
    }

    // 调用共享的应用构建和运行函数
    if let Err(e) = build_and_run_app() {
        eprintln!("应用运行错误: {}", e);
    }
}