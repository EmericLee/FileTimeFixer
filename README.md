# File Time Fixer

一个基于Tauri和Vue.js的桌面应用程序，用于查看和修复文件的时间戳信息，特别是图片的EXIF拍摄时间。

## 功能特性

- 📁 浏览和查看目录中的文件列表
- 🕒 显示文件的修改时间和大小信息
- 📷 读取图片的EXIF数据，获取拍摄时间
- 🎨 现代化的用户界面，支持中文
- 🔍 支持多种图片格式（JPEG、HEIF等）

## 技术栈

- **前端**: Vue.js 3 + Vite
- **后端**: Rust + Tauri
- **EXIF处理**: nom-exif库
- **UI**: 自定义CSS，响应式设计

## 安装和运行

### 前提条件

- Node.js (推荐v16+)
- Rust (推荐1.60+)
- npm或yarn

### 开发环境设置

1. 克隆仓库
```bash
git clone https://github.com/EmericLee/FileTimeFixer.git
cd FileTimeFixer
```

2. 安装前端依赖
```bash
npm install
```

3. 安装Rust依赖（如果尚未安装）
```bash
cd src-tauri
cargo build
cd ..
```

4. 运行开发服务器
```bash
npm run tauri dev
```

### 构建生产版本

```bash
npm run tauri build
```

构建后的应用程序将在 `src-tauri/target/release/bundle/` 目录中。

## 使用说明

1. 启动应用程序
2. 点击"选择目录"按钮或直接输入目录路径
3. 浏览文件列表，查看文件大小和修改时间
4. 对于图片文件，应用程序会尝试读取EXIF数据并显示拍摄时间

## 项目结构

```
FileTimeFixer/
├── src/                 # Vue.js前端源代码
│   ├── App.vue         # 主应用组件
│   └── main.js         # 应用入口点
├── src-tauri/           # Rust后端源代码
│   ├── src/
│   │   ├── main.rs     # Tauri应用程序入口
│   │   └── image_fixer.rs # EXIF数据处理
│   └── Cargo.toml      # Rust依赖配置
├── dist/               # 构建输出目录
├── nom-exif/           # 本地EXIF处理库
└── package.json        # Node.js依赖配置
```

## 贡献指南

欢迎提交问题和拉取请求！

1. Fork此仓库
2. 创建您的功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开一个Pull Request

## 许可证

本项目采用MIT许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- [Tauri](https://tauri.app/) - 用于构建跨平台桌面应用程序
- [Vue.js](https://vuejs.org/) - 用于构建用户界面
- [nom-exif](https://github.com/mcnote/nom-exif) - 用于解析EXIF数据

## 更新日志

### v0.1.0 (初始版本)
- 基本的文件浏览功能
- 文件信息显示（大小、修改时间）
- EXIF数据读取功能
- 现代化UI设计
