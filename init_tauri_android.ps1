<#
.SYNOPSIS
    Tauri Android 开发环境一键初始化脚本 (Windows 版)
.DESCRIPTION
    1. 安装 Node.js, Rust, Java 17, Android Studio
    2. 添加 Rust Android 编译目标
    3. 自动配置 JAVA_HOME, ANDROID_HOME 等环境变量
#>

# 检查是否以管理员运行
if (!([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "⚠️ 请以管理员身份运行此脚本！(右键 -> 以管理员身份运行)" -ForegroundColor Red
    Exit
}

Write-Host "🚀 开始初始化 Tauri Android 开发环境..." -ForegroundColor Cyan

# --- 1. 安装基础工具 (Winget) ---
Write-Host "`n[1/5] 检查并安装基础软件..." -ForegroundColor Yellow

function Install-If-Missing ($name, $id) {
    if (Get-Command $name -ErrorAction SilentlyContinue) {
        Write-Host "✅ $name 已安装" -ForegroundColor Green
    } else {
        Write-Host "⬇️ 正在安装 $name ($id)..." -ForegroundColor Cyan
        winget install --id $id -e --source winget --accept-source-agreements --accept-package-agreements
    }
}

# 安装 Node.js LTS
Install-If-Missing "node" "OpenJS.NodeJS.LTS"

# 安装 Java 17 (Tauri v2 Android 必须)
# 检查是否有 Java 17
$javaVer = java -version 2>&1 | Out-String
if ($javaVer -match "version.*17") {
    Write-Host "✅ Java 17 已安装" -ForegroundColor Green
} else {
    Write-Host "⬇️ 未检测到 Java 17，正在安装 Eclipse Temurin JDK 17..." -ForegroundColor Cyan
    winget install --id "EclipseAdoptium.Temurin.17" -e --source winget
}

# 安装 Android Studio
if (Test-Path "$env:ProgramFiles\Android\Android Studio\bin\studio64.exe") {
    Write-Host "✅ Android Studio 已安装" -ForegroundColor Green
} else {
    Write-Host "⬇️ 正在安装 Android Studio..." -ForegroundColor Cyan
    winget install --id "Google.AndroidStudio" -e --source winget
}

# 安装 Rust (如果没装)
if (Get-Command "rustup" -ErrorAction SilentlyContinue) {
    Write-Host "✅ Rustup 已安装" -ForegroundColor Green
} else {
    Write-Host "⬇️ 正在安装 Rust..." -ForegroundColor Cyan
    winget install --id "Rustlang.Rustup" -e --source winget
}

# --- 2. 配置 Rust Android 目标 ---
Write-Host "`n[2/5] 添加 Rust Android 编译目标..." -ForegroundColor Yellow
# 刷新环境变量以便识别刚安装的 rustup
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
Write-Host "✅ Rust Android Targets 添加完成" -ForegroundColor Green

# --- 3. 配置 JAVA_HOME ---
Write-Host "`n[3/5] 配置 JAVA_HOME 环境变量..." -ForegroundColor Yellow
$jdkPath = Get-ChildItem -Path "$env:ProgramFiles\Eclipse Adoptium" -Filter "jdk-17*" | Select-Object -First 1
if ($jdkPath) {
    $fullJdkPath = $jdkPath.FullName
    [System.Environment]::SetEnvironmentVariable("JAVA_HOME", $fullJdkPath, [System.EnvironmentVariableTarget]::User)
    Write-Host "✅ JAVA_HOME 设置为: $fullJdkPath" -ForegroundColor Green
} else {
    Write-Host "⚠️ 未找到 JDK 17 安装路径，请手动设置 JAVA_HOME" -ForegroundColor Red
}

# --- 4. 配置 Android 环境变量 ---
Write-Host "`n[4/5] 配置 Android 环境变量..." -ForegroundColor Yellow
# 默认 Android SDK 路径
$androidSdkPath = "$env:LOCALAPPDATA\Android\Sdk"

# 设置 ANDROID_HOME
[System.Environment]::SetEnvironmentVariable("ANDROID_HOME", $androidSdkPath, [System.EnvironmentVariableTarget]::User)
Write-Host "✅ ANDROID_HOME 设置为: $androidSdkPath" -ForegroundColor Green

# 将 platform-tools 加入 PATH (为了用 adb)
$userPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::User)
$adbPath = "$androidSdkPath\platform-tools"
if ($userPath -notlike "*$adbPath*") {
    [System.Environment]::SetEnvironmentVariable("Path", "$userPath;$adbPath", [System.EnvironmentVariableTarget]::User)
    Write-Host "✅ 已将 ADB ($adbPath) 添加到 PATH" -ForegroundColor Green
} else {
    Write-Host "✅ ADB 路径已存在" -ForegroundColor Green
}

# --- 5. NDK 提示 ---
Write-Host "`n[5/5] ⚠️ 关键步骤提示" -ForegroundColor Yellow
Write-Host "-----------------------------------------------------"
Write-Host "脚本无法自动同意 SDK 协议，请务必手动执行以下步骤："
Write-Host "1. 打开 Android Studio。"
Write-Host "2. 进入 SDK Manager -> SDK Tools。"
Write-Host "3. 勾选并安装以下三项："
Write-Host "   - [x] Android SDK Command-line Tools"
Write-Host "   - [x] NDK (Side by side)"
Write-Host "   - [x] CMake"
Write-Host "4. 安装完成后，重启电脑或重启 VS Code。"
Write-Host "-----------------------------------------------------"
Write-Host "🎉 脚本执行完毕！" -ForegroundColor Cyan
