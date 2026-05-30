# SlaySP2Manager - macOS & Arch Linux

> Slay's Spell 2 Mod Manager - 原生 macOS 和 Arch Linux 版本

[![Release](https://img.shields.io/github/v/release/fewftybet/slaySP2Manager-for-mac)](https://github.com/fewftybet/slaySP2Manager-for-mac/releases)
[![License](https://img.shields.io/github/license/fewftybet/slaySP2Manager-for-mac)](LICENSE)

## 📥 下载安装 / Download

### macOS 用户

**最新版本**: v0.9.0 (2026-05-30)

从 [GitHub Releases](https://github.com/fewftybet/slaySP2Manager-for-mac/releases) 下载：
- **SlaySP2Manager_0.9.0.dmg** - macOS 安装包 (8.6 MB)

**安装步骤：**
1. 下载 `.dmg` 文件
2. 双击打开 DMG
3. 将应用拖拽到 Applications 文件夹
4. 启动应用

**系统要求**: macOS 10.15+ (支持 Apple Silicon 和 Intel)

### Arch Linux 用户

详见 [Arch/AUR 安装说明](./packaging/arch/README)

---

## 原始项目说明

Arch Linux packaging and Linux compatibility fork of [wakaka6/SlaySP2Manger](https://github.com/wakaka6/SlaySP2Manger).

This fork focuses on:

- Arch/AUR packaging
- Linux game path detection
- Linux save path support
- Linux card compendium resource loading
- **macOS native builds** (DMG packages)

## Attribution

Original project: [wakaka6/SlaySP2Manger](https://github.com/wakaka6/SlaySP2Manger)

Original project copyright belongs to its respective authors and contributors. This fork preserves the original MIT license in [`LICENSE`](./LICENSE).

This repository is not an official upstream release unless explicitly stated by the upstream maintainers.



## GitHub Actions - macOS 自动打包

本项目配置了 GitHub Actions 工作流，可自动构建 macOS 原生应用并创建 Release。

### 触发条件

- **自动触发**: 推送 `v*` 格式的标签（如 `v1.0.0`）
- **手动触发**: 在 GitHub Actions 页面手动运行

### 使用方法

#### 1. 创建并推送版本标签

```bash
# 创建新版本标签
git tag v1.0.0

# 推送到远程仓库（会自动触发构建）
git push origin v1.0.0
# 或者
git push main v1.0.0
```

#### 2. 手动触发构建

1. 进入 GitHub 仓库页面
2. 点击 **Actions** 标签
3. 选择 **Build macOS Release** 工作流
4. 点击 **Run workflow** 按钮
5. 选择分支后点击 **Run workflow**

### 构建产物

每次构建会生成以下文件：

- **Prompt Manager.dmg** - macOS 安装镜像（推荐下载）
- **Prompt Manager.app** - 应用程序包

构建完成后，文件会自动上传到：
- **GitHub Release**: 在 Releases 页面下载
- **Artifacts**: 在 Actions 页面下载（保留 30 天）

### 系统要求

- macOS 10.15 或更高版本
- Python 3.x（GitHub Actions 自动配置）

### 技术栈

- **语言**: Python 3
- **GUI 框架**: PyObjC + Cocoa (AppKit)
- **数据库**: SQLite
- **打包工具**: 自定义脚本 + GitHub Actions

### 本地构建

如需在本地构建，运行：

```bash
# 安装依赖
pip3 install pyobjc-core pyobjc-framework-Cocoa

# 执行打包脚本
./build_native.sh

# 构建产物位于 dist_native/ 目录
```

### 工作流配置

GitHub Actions 配置文件位于：`.github/workflows/build-macos.yml`

主要步骤：
1. 检出代码
2. 设置 Python 环境
3. 安装 PyObjC 依赖
4. 执行打包脚本
5. 验证构建产物
6. 创建 GitHub Release（仅标签触发时）
7. 上传构建产物到 Artifacts
