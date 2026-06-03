<div align="center">
<img src="src-tauri/icons/icon.ico" width="100" height="100" alt="SlaySP2Manager Logo" />

# SlaySP2Manager

**杀戮尖塔2 Mod管理器 macOS版 | Slay the Spire 2 Mod Manager for Mac**

基于 Rust + Tauri + React 构建 — 快速、安全、全自动化管理。

> 🔍 **搜索关键词**: 杀戮尖塔mod管理器 mac、杀戮尖塔2 mod管理器、slay the spire 2 mod manager mac、杀戮尖塔模组管理器、sts2 mod manager

[![GitHub release](https://img.shields.io/github/v/release/fewftybet/slaySP2Manager-for-mac?style=flat-square&color=C9A84C)](https://github.com/fewftybet/slaySP2Manager-for-mac/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/fewftybet/slaySP2Manager-for-mac/release.yml?style=flat-square&label=Build)](https://github.com/fewftybet/slaySP2Manager-for-mac/actions)
[![License](https://img.shields.io/github/license/fewftybet/slaySP2Manager-for-mac?style=flat-square&color=95A5A6)](LICENSE)

**macOS 版本** | [原始项目](https://github.com/wakaka6/SlaySP2Manger)

</div>

---

## 📥 下载与安装

### macOS 用户

**最新版本**: v0.9.1 (2026-06-03)

从 [GitHub Releases](https://github.com/fewftybet/slaySP2Manager-for-mac/releases) 下载：
- **SlaySP2Manager_0.9.1_aarch64.dmg** - macOS 安装包 (Apple Silicon)

**安装步骤：**
1. 下载 `.dmg` 文件
2. 双击打开 DMG
3. 将应用拖拽到 Applications 文件夹
4. 启动应用

**系统要求**: 
- macOS 10.15 (Catalina) 或更高版本
- Apple Silicon (M1/M2/M3) 或 Intel x86_64
- 至少 4GB RAM

---

## 🎯 我们解决了什么问题

给《杀戮尖塔 2》装 Mod 很麻烦——我自己也经历过：

- 🔍 **游戏目录在哪？** 第一次装 Mod 的玩家常常花 20 分钟寻找游戏路径和存档路径。
- 🔀 **浏览器和文件夹来回切。** 在 Nexus Mods 上找到想装的 Mod，下载、解压、复制到不知对不对的目录，再祈祷游戏不崩。
- 💥 **悄无声息的冲突。** 两个 Mod 覆盖同一个文件，没有任何提示，游戏直接挂了，却不知道问题出在哪。
- 💾 **存档焦虑。** 想体验一次模组玩法，却害怕原版存档被覆盖，进退两难。
- 🔙 **没有撤销。** 更新出错之后，没有简单的回滚方式。

**SlaySP2Manager** 把上面这些焦虑场景，变成一个安静、专注、一窗搞定的操作流程。

---

## ✨ 功能概览

### 📦 模组库
- **扫描并展示**本地已安装的所有 Mod，一目了然
- 单击即可**启用 / 禁用 / 卸载** Mod
- **从 ZIP 安装** — 拖拽文件或手动选择；应用读取 manifest、检测冲突，在写入之前显示预览
- **冲突检测** — 在问题发生前高亮显示 Mod 之间的文件级冲突
- **操作日志** — 每次安装、更新、卸载都有清晰的记录

### 🔍 发现（Nexus Mods 集成）
- **在应用内搜索** Nexus Mods 上的 STS2 Mod，无需切换浏览器
- **Mod 详情面板** — 描述、作者、版本、点赞数、标签
- **多语言友好** — 支持在应用内翻译 Mod 描述，消除阅读障碍
- **在 Nexus 中打开** — 需要时可直接跳转到完整的 Nexus 页面
- **下载队列** — 全程可见，切换页面后不会消失
- 需要免费的 Nexus Mods API Key（应用内有获取引导教程）

### 🗂️ 预设
- 创建 **多套本地 Mod 预设**（例如"原版兼容""全力混乱"）
- 安全切换预设 — 应用在切换前会自动校验完整性
- 支持复制、重命名预设
- **Profiles 自动保存** — 编辑现有配置档时，模组选中变更会自动持久化
- **整合包分享**：将预设连同所有关联 Mod 打包为 `.zip` 整合包（含 `.spm` 清单），一键分享给好友
- **整合包导入**：点击按钮或拖拽整合包到窗口即可导入，自动检测冲突并提供逐 Mod 的「跳过 / 替换」选择
- 在模组库页面 **一键保存当前启用的 Mod 为新预设**

### 🃏 图鉴
- **原生卡牌图鉴** — 画廊式浏览《杀戮尖塔 2》卡牌，支持详情查看、升级态切换与本地化文本展示
- **游戏资源直出** — 从本地游戏提取卡图、边框、横幅、能量图标与标题字体，展示更贴近游戏内卡面
- **本地生成与缓存** — 刷新资源时根据已识别的游戏路径重建图鉴元数据和原生资源缓存
- **长列表浏览优化** — 提供吸顶筛选栏、折叠筛选区和回到顶部按钮，筛卡更顺手

### 💾 存档管理
- 清晰区分**原版存档槽位**和**模组存档槽位**
- 原版与模组存档之间**双向复制**（操作前显示将覆盖哪些内容）
- **存档配对同步** — 将任意原版槽位与模组槽位关联，开启自动同步后根据修改时间双向同步（类似 rsync），支持跨槽位配对
- 卡片之间**可视化连线**，清晰展示配对关系；点击连线中间的 × 按钮即可解除
- **高风险操作前自动备份** — 不会悄悄覆盖任何东西
- **备份列表与恢复** — 浏览历史备份，一键恢复

### ☁️ Steam 云存档同步
- **自动发现**当前 Steam 账号的云存档目录，无需手动配置
- 一键**上传至云端**（本地 → Cloud）或**下载至本地**（Cloud → 本地），原版与模组存档全量同步
- 每次云端操作前**自动创建完整备份**，确保数据安全
- 支持直接**打开云存档文件夹**，方便手动查看和管理

### ⚙️ 设置与诊断
- 首次启动自动检测游戏目录
- 可配置下载目录和 Nexus API Key
- **应用内教程**：引导获取 API Key，无需离开应用
- 诊断页面：校验游戏路径、存档路径和 Mod 目录健康情况

---

## 🛠️ 本地开发

### 环境要求

| 工具 | 版本要求 |
|------|---------|
| Node.js | 20+ |
| Rust | stable（通过 `rustup` 安装） |
| macOS | 10.15+ |

### 启动开发模式

```bash
# 安装前端依赖
npm install

# 启动 Tauri 开发模式（热重载）
npm run tauri:dev
```

### 构建发布包

```bash
# macOS 构建
npm run tauri build
# 产物目录：src-tauri/target/release/bundle/dmg/
```

---

## 🤝 参与贡献

欢迎提交 Bug 报告、功能建议和 Pull Request！

1. Fork 本仓库
2. 创建你的分支：`git checkout -b feat/my-feature`
3. 提交改动：`git commit -m 'feat: 新增某功能'`
4. 推送并发起 Pull Request

提交信息请遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范。

---

## 🙏 致谢

### 原始项目

本项目基于 **[wakaka6/SlaySP2Manger](https://github.com/wakaka6/SlaySP2Manger)** 进行移植和扩展。

感谢原项目作者和贡献者们的出色工作：

<div align="center">

| 贡献者 | 贡献数 |
|--------|--------|
| [![wakaka6](https://avatars.githubusercontent.com/u/48764488?v=4)](https://github.com/wakaka6) | 72 |
| [![Copilot](https://avatars.githubusercontent.com/in/1143301?v=4)](https://github.com/copilot) | 6 |

</div>

### 特别感谢

- **@wakaka6** - 原始项目的创建者和主要维护者
- **所有原始项目的贡献者** - 为社区做出了重要贡献
- **Nexus Mods** - 提供 Mod 资源和 API 支持
- **Tauri 团队** - 提供优秀的跨平台框架
- **《杀戮尖塔 2》社区** - 持续创作优质的 Mod 内容

### 本项目贡献者

感谢所有为本 macOS 版本做出贡献的朋友们！

---

## 📄 许可证

MIT License — 详见 [LICENSE](LICENSE) 文件。

原始项目的版权和许可证同样适用于本项目的相应部分。

---

<div align="center">

**macOS 版本**

为《杀戮尖塔 2》社区打造

用 ❤️ 制作

</div>
