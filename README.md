<div align="center">

# 🛠️ hachimi-edge-gala

**把 Hachimi-Edge 的资源注入能力移植到 BestSoccer（galasports）**

![仓库](https://img.shields.io/badge/仓库-xf8410-8B5CF6?style=flat-square) ![分支](https://img.shields.io/badge/分支-2-10B981?style=flat-square) ![版本](https://img.shields.io/badge/版本-2-F59E0B?style=flat-square) ![CI](https://img.shields.io/badge/CI-3-3B82F6?style=flat-square)

</div>

---
> 📌 **一句话定位**：把 Hachimi-Edge 的资源注入能力移植到 BestSoccer（galasports）

## 🧭 项目定位

本项目是 <b>Hachimi-Edge</b> 的分支改造版：保留原框架「游戏启动时注入自定义资源与补丁」的核心机制，把目标应用从赛马娘切换为 BestSoccer（com.galasports.legendaryfootball.cn），实现足球游戏的界面与资源定制。

## ✨ 核心功能
- 游戏启动期注入自定义资源与补丁，无需修改 APK 本体- Rust 实现，与上游 Hachimi-Edge 保持同构，方便跟随上游演进- 按包名 galasports 适配资源路径与目标应用检测

## 🌿 分支导览（共 2 个分支全览）

<details open>
<summary><b>点击收起/展开全部分支用途说明</b></summary>

| 分支 | 用途说明 |
|---|---|
| `main` | 主干：BestSoccer 适配的改造代码 |
| `auto-security-fix` | 自动安全修复分支（机器人维护） |

</details>

## 🏷️ 版本历史

现有 v0.29.0 / v0.29.1 两个版本，跟随上游 Hachimi-Edge 版本号，通过 Create Release 流水线产出。

完整版本列表 ➡️ [Releases 页](../../releases)

## ⚙️ CI 流水线（共 3 条）

| 流水线 | 用途说明 |
|---|---|
| Daily Cargo Audit | 每日依赖安全审计（cargo audit） |
| Clippy Check | Rust 代码静态检查 |
| Create Release | 发布构建，产出安装包到 Releases |


---

## 📜 历史介绍存档

> 以下为仓库原有介绍，**内容未删改**，仅移入存档区（新版介绍以本页上方为准）。

<details>
<summary><b>点击展开原 README</b></summary>

<img align="left" width="80" height="80" src="assets/icon.png">

# Hachimi Edge

English | [简体中文](README-zh_cn.md) | [繁體中文](README-zh_tw.md)

[![Discord server](https://dcbadge.limes.pink/api/server/https://discord.gg/YjBgmuqqYr)](https://discord.gg/YjBgmuqqYr)

Game enhancement and translation mod for UM:PD.

<img width="100%" height="100%" src="assets/screenshot-1.png">
<img width="100%" height="100%" src="assets/screenshot-2.png">

# ⚠️ Please don't link to this repo or Hachimi's website
We understand that you want to help people install Hachimi and have a better experience playing the game. However, this project is inherently against the game's TOS and The Game Developer most definitely wants it gone if they were ever to learn about it.

While sharing in your self-managed chat services and through private messaging is fine, we humbly ask that you refrain from sharing links to this project on public facing sites, or to any of the tools involved.

Or share them and ruin it for the dozens of Hachimi users. It's up to you.

### If you're going to share it anyways
Do what you must, but we would respectfully request that you try to label the game as "UM:PD" or "The Honse Game" instead of the actual name of the game, to avoid search engine parsing.

# Features
- **High quality translations:** Hachimi comes with advanced translation features that help translations feel more natural (plural forms, ordinal numbers, etc.) and prevent introducing jank to the UI. It also supports translating most in-game components; no manual assets patching needed!

    Supported components:
    - UI text
    - master.mdb (skill name, skill desc, etc.)
    - Race story
    - Main story/Home dialog
    - Lyrics
    - Texture replacement
    - Sprite atlas replacement

    Additionally, Hachimi does not provide translation features for only a single language; it has been designed to be fully configurable for any language.

- **Easy setup:** Just plug and play. All setup is done within the game itself, no external application needed.
- **Translation auto update:** Built-in translation updater lets you play the game as normal while it updates, and reloads it in-game when it's done, no restart needed!
- **Built-in GUI:** Comes with a config editor so you can modify settings without even exiting the game!
- **Graphics settings:** You can adjust the game's graphics settings to make full use of your device's specs, such as FPS unlocking and resolution scaling.
- **Cross-platform:** Designed from the ground up to be portable, with Windows and Android support.

# Installation
Please see the [Getting started](https://hachimi.noccu.art/docs/hachimi/getting-started.html) page.

# Special thanks
These projects have been the basis for Hachimi's development; without them, Hachimi would never have existed in its current form:

- [Trainers' Legend G](https://github.com/MinamiChiwa/Trainers-Legend-G)
- [umamusume-localify-android](https://github.com/Kimjio/umamusume-localify-android)
- [umamusume-localify](https://github.com/GEEKiDoS/umamusume-localify)
- [Carotenify](https://github.com/KevinVG207/Uma-Carotenify)
- [umamusu-translate](https://github.com/noccu/umamusu-translate)
- [frida-il2cpp-bridge](https://github.com/vfsfitvnm/frida-il2cpp-bridge)

# License
[GNU GPLv3](LICENSE)


</details>
