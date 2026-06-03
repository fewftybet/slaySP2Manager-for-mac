# SlaySP2Manager for macOS

> 杀戮尖塔2 Mod管理器 macOS版 | Slay the Spire 2 Mod Manager for Mac

[![Release](https://img.shields.io/github/v/release/fewftybet/slaySP2Manager-for-mac)](https://github.com/fewftybet/slaySP2Manager-for-mac/releases)
[![License](https://img.shields.io/github/license/fewftybet/slaySP2Manager-for-mac)](LICENSE)

## 📥 下载安装 / Download

### macOS 用户

从 [GitHub Releases](https://github.com/fewftybet/slaySP2Manager-for-mac/releases) 下载最新的 DMG 安装包：

- **SlaySP2Manager_0.9.1_aarch64.dmg** - macOS 安装包 (Apple Silicon)

**安装步骤：**
1. 下载 `.dmg` 文件
2. 双击打开 DMG
3. 将 `SlaySP2Manager.app` 拖拽到 `Applications` 文件夹
4. 启动应用

### 系统要求

- **操作系统**: macOS 10.15 (Catalina) 或更高版本
- **架构**: Apple Silicon (M1/M2/M3) 或 Intel x86_64
- **内存**: 至少 4GB RAM

## ✨ 功能特性 / Features

- 🎴 **卡牌图鉴** - 浏览和管理所有卡牌资源
- 💾 **存档管理** - 轻松备份和恢复游戏存档
- 🔧 **MOD 管理** - 一键安装和管理 MOD
- 📦 **配置文件管理** - 支持多配置文件切换
- 🌐 **中英双语** - 完整的国际化支持

## 🚀 使用指南 / Usage

### 首次使用

1. 启动 SlaySP2Manager
2. 应用会自动检测游戏安装路径
3. 如果未检测到，请手动设置路径

### 管理 MOD

1. 进入「MOD 库」页面
2. 浏览可用的 MOD
3. 点击安装按钮
4. 选择要应用的配置文件

### 备份存档

1. 进入「存档」页面
2. 选择要备份的存档
3. 点击「备份」按钮
4. 需要时可从备份恢复

## 🛠️ 技术栈 / Tech Stack

- **前端**: React 18 + TypeScript + Vite 6
- **后端**: Rust + Tauri 2
- **构建工具**: Node.js 26, Cargo
- **打包格式**: macOS DMG

## 📦 本地构建 / Local Build

如果你想在本地构建项目：

### 环境要求

- Node.js 20+ 
- Rust 1.70+
- Xcode Command Line Tools

### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/fewftybet/slaySP2Manager-for-mac.git
cd slaySP2Manager-for-mac

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```

构建完成后，DMG 安装包位于：
`src-tauri/target/release/bundle/dmg/`

## 📝 更新日志 / Changelog

### v0.9.1 (2026-06-03)

**修复:**
- 修复无法打开文件夹的 bug：添加 macOS 专属 Steam 库路径和游戏可执行文件检测
- 修复图鉴无法使用的 macOS 兼容错误：修复资源路径、PCK 文件名、sts2.dll 搜索路径
- 修复启动游戏的兼容性错误：修复游戏根目录和可执行文件路径解析
- 修复窗口无法拖动：使用 macOS 原生 Overlay 标题栏

### v0.9.0 (2026-05-30)

**新增:**
- ✅ 使用 Node.js 26 构建
- ✅ 修复 GitHub Actions Node.js 20 弃用问题
- ✅ 更新 GitHub Actions 到最新版本 (v5)
- ✅ 优化构建流程

**修复:**
- 修复了部分用户遇到的游戏路径检测问题
- 改进了 MOD 安装的稳定性

[完整更新日志](CHANGELOG.md)

## 🤝 贡献 / Contributing

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的修改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

## 📄 许可证 / License

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关链接 / Links

- [原始项目](https://github.com/wakaka6/SlaySP2Manger)
- [问题反馈](https://github.com/fewftybet/slaySP2Manager-for-mac/issues)
- [讨论区](https://github.com/fewftybet/slaySP2Manager-for-mac/discussions)

## 🙏 致谢 / Acknowledgments

感谢所有为这个项目做出贡献的开发者和用户！

---

**注意**: 本项目是 SlaySP2Manager 的 macOS 分支版本，专注于为 macOS 用户提供原生体验。
