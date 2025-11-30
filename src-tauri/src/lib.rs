// 库目标源文件，避免main.rs被用于多个构建目标

// 导入必要的模块
mod image_fixer;

// 共享的应用构建和运行函数
pub  fn build_and_run_app() -> Result<(), tauri::Error> {
    // 创建并运行Tauri应用
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            image_fixer::scan_directory,
            image_fixer::demonstrate_emit_events
        ])
        .run(tauri::generate_context!())?; // 使用 ? 传播错误，避免未使用的 Result
    Ok(())
}

// 为移动平台提供入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Android构建必需的入口点函数
    println!("File Time Fixer 移动平台入口点启动");
    
    // 调用共享的应用构建和运行函数
    if let Err(e) = build_and_run_app() {
        // 使用标准错误输出
        eprintln!("移动平台应用启动失败: {}", e);
        std::process::exit(1);
    }
}

// 导出image_fixer模块中的函数供其他模块使用
pub use image_fixer::{scan_directory, select_directory, demonstrate_emit_events};

// 导出共享函数供main.rs使用
// 移除对 build_and_run_app 的 pub(crate) use，避免与 run 函数冲突
