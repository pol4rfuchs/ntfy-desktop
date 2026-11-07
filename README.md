<div align="center">
<h6>Ntfy.sh desktop client for Windows, Linux, and MacOSX</h6>
<h1>♾️ ntfy-desktop ♾️</h1>

<br />

<p>A ntfy.sh desktop client built with Tauri (Rust) which supports Windows, Linux, and MacOSX. This client rests in your taskbar tray and allows you to receive push notifications to your desktop without requiring you to leave your browser open.</p>

<p float="left">
  <img style="padding-right:15px;" src="https://github.com/user-attachments/assets/b6a34bc3-dbbf-4249-b3d2-d113a21cca66" width="300" />
  <img src="https://github.com/user-attachments/assets/4f842360-bc94-46eb-8660-17f221fe745d" width="300" /> 
</p>

<p float="left">
  <img style="padding-right:15px;" src="https://github.com/user-attachments/assets/ebe0c02c-336f-4e95-b0f9-24c5b853f0e4" width="300" />
  <img src="https://github.com/user-attachments/assets/31c0a712-f596-44b1-a30d-fdbe579d9df6" width="300" /> 
</p>

<p float="left">
  <img style="padding-right:15px;" src="https://github.com/user-attachments/assets/ce32d901-b35b-48e5-85b5-3cf82ae09b1e" width="300" />
  <img src="https://github.com/user-attachments/assets/a04f6222-9a29-40c3-a2d2-82bd8f6f4c09" width="300" /> 
</p>

<br />

</div>

<div align="center">

<!-- prettier-ignore-start -->
[![Version][github-version-img]][github-version-uri]
[![Build Status][github-build-img]][github-build-uri]
[![Downloads][github-downloads-img]][github-downloads-uri]
[![Size][github-size-img]][github-size-uri]
[![Last Commit][github-commit-img]][github-commit-uri]
[![Contributors][contribs-all-img]](#contributors-)
<!-- prettier-ignore-end -->

</div>

<br />

---

<br />

- [About](#about)
  - [What is Ntfy](#what-is-ntfy)
  - [Ntfy Desktop Features](#ntfy-desktop-features)
  - [Self-hosted vs Ntfy.sh](#self-hosted-vs-ntfysh)
  - [Architecture Outline](#architecture-outline)
- [Usage](#usage)
  - [Key Binds](#key-binds)
  - [CLI Arguments](#cli-arguments)
- [Build](#build)
  - [Requirements](#requirements)
  - [Build Script](#build-script)
  - [Run From Source](#run-from-source)
  - [GitHub Workflows](#github-workflows)
  - [Releasing](#releasing)
- [Tests](#tests)
- [Verify Downloads](#verify-downloads)
  - [Check a Download](#check-a-download)
    - [Windows](#windows)
    - [Linux](#linux)
    - [MacOS](#macos)
  - [First Start Warnings](#first-start-warnings)
- [Dependencies](#dependencies)
- [Contributors ✨](#contributors-)


<br />

---

<br />

# About

This project allows you to access the official free / paid notification service [ntfy.sh](https://ntfy.sh/), or your own self-hosted version of ntfy from within a desktop application which uses the system webview (Tauri / Rust) as the wrapper.

<br />

## What is Ntfy

[ntfy.sh](https://ntfy/) (pronounced "notify") is a simple HTTP-based pub-sub notification service. With [ntfy](https://ntfy/), you can send notifications to your phone or desktop via scripts from any computer, without having to sign up or pay any fees. If you'd like to run your own instance of the service, you can easily do so since ntfy is open source.

To install Ntfy on your system, visit the following links:

<div align="center">

[![View](https://img.shields.io/badge/%20-%20Download%20Ntfy.sh%20-%20%231F85DE?style=for-the-badge&logo=github&logoColor=FFFFFF)](https://github.com/binwiederhier/ntfy/releases) [![Download](https://img.shields.io/badge/%20-%20View%20Documentation-%20%23de2343?style=for-the-badge&logo=github&logoColor=FFFFFF)](https://docs.ntfy.sh/install/)

</div>

<br />

## Ntfy Desktop Features

- Supports both the official ntfy.sh website / service or your own self-hosted instance
  - To self-host, you must install Ntfy server on a local machine.
  - View docs at https://docs.ntfy.sh/install/
- Two modes for minimizing app, configure in settings
  1. Close button exits app completely; OR
  2. Close button sends app to tray. Right-click tray icon to quit / show app
- Start app minimized
- Start at login (Settings → General → Start at Login); starts minimized in the tray
- Shortcut key-binds
  - Can disable the keyboard shortcuts
- Receive push notifications from ntfy server to desktop
  - Ability to adjust polling rate
  - Modify `datetime` format
  - Notifications (two options):
    1. Persistent (sticky) notifications which require user interaction to clear
    2. Timed notifications which disappear after X seconds
  - Topic filtering & support
    - Defaults: `news, announcements`
- Supports Ntfy API token
- Taskbar unread message count / count
  - Where the platform has no badge API, the count is shown in the tray tooltip
- Notifications show the ntfy message title when a message has one
- API token is stored in the system keychain (Windows Credential Manager, macOS Keychain, Secret Service on Linux)
- Includes [command-line arguments](#cli-arguments)

<br />
<br />

## Self-hosted vs Ntfy.sh

To use this desktop client, you will be required to either have an Ntfy.sh account, or you must host your own instance of the Ntfy.sh server to pull notifications from. 

You can install your own self-hosted copy of Ntfy server from:

- https://github.com/binwiederhier/ntfy/releases

<br />

Be aware that the official [ntfy.sh](https://ntfy/) website will **rate-limit** users who have not purchased a paid package. Out of box, this ntfy-desktop client polls for new notifications every `30 seconds`; if you are on the free plan and decrease this timer in the desktop client settings, you will get an error saying that you have gone over your rate-limit. 

<br />

If you are self-hosting your own copy of Ntfy, you must open the Ntfy Desktop client, click the top menu item **App**, go down to **Settings** and then click **Instance**. You must set your instance URL to your personal self-hosted instance. To set your instance URL back to the official ntfy server, clear the URL box and save.

<p align="center"><img style="width: 80%;text-align: center;" src="docs/img/README/selfhost.gif"></p>

<br />

You can set the polling rate lower without any limitations or rate limits:

<p align="center"><img style="width: 80%;text-align: center;" src="docs/img/README/polling.gif"></p>

<br />

## Architecture Outline

The official releases are built for **Windows x64** (installer `.exe` and `.msi`), **Linux amd64** (`.deb`, `.rpm`, `.AppImage`) and **macOS universal** (`.dmg`, Intel and Apple Silicon). Other architectures can be built from source on a machine of that architecture. The table below lists common architectures for reference:

<br />

| Architecture | Bits | Devices / Examples                                             |
| ------------ | ---- | -------------------------------------------------------------- |
| **ARMv7l**   | 32   | Raspberry Pi 2 & 3 (32-bit), Nexus 7, PS Vita (ARM Cortex-A9)  |
| **ARMv6l**   | 32   | Raspberry Pi 1, Pi Zero, iPhone 3G                             |
| **ARM64**    | 64   | Apple M1/M2/M3 Macs, Raspberry Pi 4 (64-bit OS), iPhone 5s+    |
| **AMD64**    | 64   | Modern PCs, PS5, Xbox Series X, Steam Deck                     |
| **I386**     | 32   | 386-era PCs, Windows 95 machines, Compaq Deskpro, old netbooks |
| **IA32**     | 32   | Same as i386; Intel Atom netbooks, Pentium 4 PCs               |

<br />

---

<br />

# Usage

This section explains how to use ntfy-desktop once you have it ready to go on your system.

<br />

## Key Binds
The following keybinds can be used within ntfy-desktop:

<br />

| Key(s) | Description |
| --- | --- |
| `CTRL + R` | Refresh page |
| `CTRL + Q` | Quit application |
| `CTRL + M` | Minimize to tray |
| `CTRL + =` | Zoom in |
| `CTRL + -` | Zoom out |
| `CTRL + 0` | Zoom reset |
| `CTRL + SHIFT + I` | Developer tools |
| `F12` | Developer tools |
| `CTRL + G` | Show General settings window |
| `CTRL + I` | Show URL / Instance window |
| `CTRL + T` | Show API Token settings window |
| `CTRL + SHIFT + T` | Show Topics settings window |
| `CTRL + N` | Show Notifications settings window |

<br />

> [!NOTE]
> Hotkeys are disabled by default. To enable hotkeys, select **App** in the top menu, and select **Settings** -> **General**.
> 
> Enable `Allow usage of hotkeys to navigate`

<br />
<br />

## CLI Arguments

This client allows you to utilize the following command-line arguments with ntfy-desktop:

<br />

| Argument | Description | Available as setting |
| --- | --- | --- |
| `--hidden` | Start app hidden in tray, suitable for auto-starting on system login/boot | ✅ |
| `--hotkeys` | Start app with hotkeys enabled | ✅ |
| `--terminate` | Top-right close button will completely exit app instead of minimize to tray | ✅ |
| `--devtools` | Start app with developer tools in `App` menu | ✅ |
| `--env <env_name>` | Define startup environment (`production`, `development`, `test`) | ❌ |
| `--help`, `-h` | Print the argument list and exit | ❌ |

<br />

If you are running ntfy-desktop from source, you can pass arguments using the following example:

```shell ignore
cd src-tauri
cargo run -- --hidden --hotkeys
```

<br />

If using the pre-built binaries, pass arguments by opening your terminal / command prompt, and passing the arguments using:

```shell
# Windows (default install path)
"%LOCALAPPDATA%\Ntfy Desktop\ntfy-desktop.exe" --hidden --hotkeys

# Linux
ntfy-desktop --hidden --hotkeys

# MacOS
"/Applications/Ntfy Desktop.app/Contents/MacOS/ntfy-desktop" --hidden --hotkeys
```

<br />

---

<br />

# Build

Ntfy Desktop is built with [Tauri 2](https://tauri.app) (Rust). The settings file (`prefs.json`) is the same as in the old Electron version, so existing settings carry over. **The API token has to be entered once again**, because it is now stored in the system keychain.

<br />

## Requirements

| System | Needs |
| --- | --- |
| **All** | [Rust](https://rustup.rs) (stable) |
| **Windows** | "Desktop development with C++" from the Visual Studio Build Tools; WebView2 (preinstalled on Windows 11) |
| **Linux** | `sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libssl-dev libdbus-1-dev pkg-config` |
| **macOS** | Xcode command line tools: `xcode-select --install` |

<br />

## Build Script

```shell
git clone https://github.com/pol4rfuchs/ntfy-desktop.git
cd ntfy-desktop

# Linux / macOS
./build.sh

# Windows (PowerShell or Command Prompt)
build.bat
```

The script runs the unit tests and builds the installers for the operating system you are on. Tauri does not cross-compile desktop installers, so build on each target system, or use the GitHub workflow below. The installers are written to `src-tauri/target/release/bundle`.

<br />

## Run From Source

```shell
cd src-tauri
cargo run
```

<br />

## GitHub Workflows

| Workflow | Trigger | What it does |
| --- | --- | --- |
| `Build Tauri (Rust)` | manual, or a pull request touching `src-tauri/` or `ui/` | runs Rust tests and Clippy, builds installers for Windows, Linux and macOS and attaches them to the run as artifacts |
| `Build & Release` | manual | raises the version (input `BUMP`, default `auto`), commits it to `main`, builds the installers and publishes a GitHub release with checksums. See [Releasing](#releasing) |

<br />

## Releasing

Start **Actions → Build & Release → Run workflow** on the `main` branch. You do not edit any version number by hand.

| Input | Meaning |
| --- | --- |
| `BUMP` = `auto` (default) | next patch version (2.2.2 → 2.2.3), unless the current version has no release yet; then it is released as it is |
| `BUMP` = `patch` / `minor` / `major` | always raise that part (2.2.2 → 2.2.3 / 2.3.0 / 3.0.0) |
| `BUMP` = `none` | keep the current version; stops if that release already exists |
| `RC_RELEASE` on | publishes a pre-release `vX.Y.Z-rc.N` (`RC_VERSION` = N) |

The workflow writes the new version into `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock` and the file names in the README, commits that to `main` (`chore(release): bump version to …`) and builds and tags exactly that commit. A release candidate followed by a normal run with `auto` releases the same version number.

If a release run fails, use **Re-run failed jobs**. **Re-run all jobs** stops because the release already exists.

<br />

More details are in [`src-tauri/README.md`](src-tauri/README.md).

<br />

---

<br />

# Tests

## Rust Tests

```shell
cd src-tauri
cargo test
cargo clippy -- -D warnings
```

They run automatically in the `Build Tauri (Rust)` workflow.

<br />

---

<br />

# Verify Downloads

Every release contains the files `SHA256SUMS.txt` and `SHA1SUMS.txt`. They list the checksum of every installer of that release, so you can check that a download is complete and was not changed. The installers themselves are **not code-signed**: Windows SmartScreen and macOS Gatekeeper show a warning on first start (see below).

<br />

## Check a Download

Put the installer and `SHA256SUMS.txt` in the same folder. Replace the version in the file names with the one you downloaded.

<br />

### Windows

```powershell
# print the checksum of the installer
Get-FileHash .\ntfy-desktop-2.2.3-windows-x64-setup.exe -Algorithm SHA256

# show the expected value from the release; both must be identical
Select-String -Path .\SHA256SUMS.txt -Pattern "windows-x64-setup"
```

<br />

### Linux

```shell
# checks every file in the folder that is listed in SHA256SUMS.txt
sha256sum -c SHA256SUMS.txt --ignore-missing
```

Expected output: `ntfy-desktop-2.2.3-linux-amd64.deb: OK`. Any `FAILED` means the file is damaged or changed; download it again.

<br />

### macOS

```shell
# print the checksum and compare it with the line in SHA256SUMS.txt
shasum -a 256 ntfy-desktop-2.2.3-macos-universal.dmg
grep macos-universal SHA256SUMS.txt
```

<br />

## First Start Warnings

- **Windows:** SmartScreen shows "Windows protected your PC". Click **More info**, then **Run anyway**.
- **macOS:** the app is not notarized. Right-click the app, choose **Open**, and confirm.
- **Linux:** make the AppImage executable first: `chmod +x ntfy-desktop-*.AppImage`.

<br />

---

<br />

# Dependencies

Ntfy Desktop is written in Rust and relies on these main crates (see [`src-tauri/Cargo.toml`](src-tauri/Cargo.toml) for the full list):

<br />

| Crate | Purpose |
| --- | --- |
| [tauri](https://crates.io/crates/tauri) | App framework: windows, system webview, tray, menus |
| [tauri-plugin-single-instance](https://crates.io/crates/tauri-plugin-single-instance) | Only one running instance |
| [tauri-plugin-opener](https://crates.io/crates/tauri-plugin-opener) | Opens links in the default browser |
| [tauri-plugin-dialog](https://crates.io/crates/tauri-plugin-dialog) | Native message dialogs |
| [reqwest](https://crates.io/crates/reqwest) | HTTP requests to the ntfy server |
| [tokio](https://crates.io/crates/tokio) | Async runtime for the polling loop |
| [notify-rust](https://crates.io/crates/notify-rust) | Native desktop notifications |
| [keyring](https://crates.io/crates/keyring) | API token storage in the system keychain |
| [chrono](https://crates.io/crates/chrono) | Date and time formatting |

<br />

---

<br />

# Contributors ✨

We are always looking for contributors. If you feel that you can provide something useful to this project, then we'd love to review your suggestion. Before submitting your contribution, please review the following resources:

- [Pull Request Procedure](.github/PULL_REQUEST_TEMPLATE.md)
- [Contributor Policy](CONTRIBUTING.md)

<br />

Want to help but can't write code?

- Review [active questions by our community](https://github.com/pol4rfuchs/ntfy-desktop/labels/❔%20Question) and answer the ones you know.

<br />

![Alt](https://repobeats.axiom.co/api/embed/09d6197df261c2b4521ddb0f2560f78ed2811a0d.svg "Repobeats")

<br />

The following people have helped get this project going:


<br />

<div align="center">

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![Contributors][contribs-all-img]](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top"><a href="https://github.com/aetherinox"><img src="https://avatars.githubusercontent.com/u/118329232?v=4&s=40" width="80px;" alt="Aetherinox"/><br /><sub><b>Aetherinox</b></sub></a><br /><a href="https://github.com/aetherinox/noxenv/commits?author=aetherinox" title="Code">💻</a> <a href="#projectManagement-aetherinox" title="Project Management">📆</a></td>
    </tr>
  </tbody>
</table>
</div>
<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

<br />
<br />

<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->







<!-- BADGE > VERSION > GITHUB -->
  [github-version-img]: https://img.shields.io/github/v/release/pol4rfuchs/ntfy-desktop?logo=GitHub&label=Version&color=ba5225
  [github-version-uri]: https://github.com/pol4rfuchs/ntfy-desktop/releases

<!-- BADGE > GITHUB > BUILD -->
  [github-build-img]: https://img.shields.io/github/actions/workflow/status/pol4rfuchs/ntfy-desktop/build-tauri.yml?branch=main&logo=github&logoColor=FFFFFF&label=Build&color=%23278b30
  [github-build-uri]: https://github.com/pol4rfuchs/ntfy-desktop/actions/workflows/build-tauri.yml

<!-- BADGE > GITHUB > DOWNLOAD COUNT -->
  [github-downloads-img]: https://img.shields.io/github/downloads/pol4rfuchs/ntfy-desktop/total?logo=github&logoColor=FFFFFF&label=Downloads&color=376892
  [github-downloads-uri]: https://github.com/pol4rfuchs/ntfy-desktop/releases

<!-- BADGE > GITHUB > REPO SIZE -->
  [github-size-img]: https://img.shields.io/github/repo-size/pol4rfuchs/ntfy-desktop?logo=github&label=Size&color=59702a
  [github-size-uri]: https://github.com/pol4rfuchs/ntfy-desktop

<!-- BADGE > GITHUB > COMMIT -->
  [github-commit-img]: https://img.shields.io/github/last-commit/pol4rfuchs/ntfy-desktop?logo=conventionalcommits&logoColor=FFFFFF&label=Last%20Commit&color=313131
  [github-commit-uri]: https://github.com/pol4rfuchs/ntfy-desktop/commits/main/

<!-- BADGE > ALL CONTRIBUTORS -->
  [contribs-all-img]: https://img.shields.io/github/all-contributors/pol4rfuchs/ntfy-desktop?logo=contributorcovenant&color=de1f6f&label=contributors
  [contribs-all-uri]: https://github.com/all-contributors/all-contributors

<!-- prettier-ignore-end -->
<!-- markdownlint-restore -->
