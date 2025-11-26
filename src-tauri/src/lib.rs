// 库目标源文件，避免main.rs被用于多个构建目标

// 导入必要的模块
mod image_fixer;

// 为移动平台提供入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 这里可以调用main中的功能，但要避免循环依赖
    // 对于Android构建，这个函数是必需的
}

// 导出image_fixer模块中的函数供其他模块使用
pub use image_fixer::{scan_directory,  demonstrate_emit_events};