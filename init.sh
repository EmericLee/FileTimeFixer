#!/bin/bash

# Tauri + Vue + Rust + Android SDK 开发环境自动配置脚本
# 适用于 Debian 系统 trixie 版本

set -e # 遇到错误立即退出

echo "开始配置 Tauri + Vue + Rust + Android SDK 开发环境..."

# 1. 安装系统基础依赖
echo "步骤 1: 安装系统基础依赖..."
sudo apt update
sudo apt install -y \
    build-essential \
    curl \
    wget \
    file \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libssl-dev

# 2. 安装和配置 Rust 工具链
echo "步骤 2: 安装和配置 Rust 工具链..."
if ! command -v rustup &> /dev/null; then
    echo "安装 Rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rustup 已安装"
fi

# 确保 cargo 和 rustc 可用
if ! command -v cargo &> /dev/null; then
    echo "加载 Cargo 环境..."
    source "$HOME/.cargo/env"
fi
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
 
# 设置默认工具链
echo "设置默认 Rust 工具链..."
rustup default stable

# 添加 Android 交叉编译目标
rustup target add aarch64-linux-android
# rustup target add x86_64-linux-android
# rustup target add i686-linux-android
# rustup target add armv7-linux-androideabi

# 3. 安装 Node.js 和 pnpm
echo "步骤 3: 安装 Node.js 和 pnpm..."
if ! command -v node &> /dev/null; then
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt install -y nodejs
else
    echo "Node.js 已安装，跳过..."
fi

sudo npm install -g pnpm

# 4. 安装 Tauri CLI
echo "步骤 4: 安装 Tauri CLI..."
sudo npm install -g @tauri-apps/cli

# 5. 安装 Android SDK 和 NDK
echo "步骤 5: 安装 Android SDK 和 NDK..."
sudo apt install -y default-jdk  #openjdk-21-jdk

# 设置环境变量
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/26.2.11394342"
export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools"

# 将环境变量永久化
{
    echo "export ANDROID_HOME=\"$ANDROID_HOME\""
    echo "export NDK_HOME=\"$NDK_HOME\""
    echo "export PATH=\"\$PATH:\$ANDROID_HOME/cmdline-tools/latest/bin:\$ANDROID_HOME/platform-tools\""
} >> "$HOME/.bashrc"

# 下载并安装 Android Command Line Tools
mkdir -p "$ANDROID_HOME"
cd "$ANDROID_HOME"
if [ ! -f "commandlinetools-linux-9477386_latest.zip" ]; then
    wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip
    unzip commandlinetools-linux-9477386_latest.zip
    mkdir -p cmdline-tools/latest
    mv cmdline-tools/* cmdline-tools/latest/
    rm -r cmdline-tools # 移除空目录
fi

# 接受许可并安装必要组件
yes | sdkmanager --licenses
sdkmanager "platform-tools" "platforms;android-33" "build-tools;33.0.2" "ndk;26.2.11394342"

echo "环境配置完成！"

# 验证安装
echo "验证安装版本："
rustc --version
cargo --version
node --version
pnpm --version
tauri --version

echo "所有工具链已安装完毕。"
echo "请重启终端或执行 'source ~/.bashrc' 使环境变量生效。"


