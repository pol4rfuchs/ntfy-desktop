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
[![Build Status][github-tests-img]][github-tests-uri]
[![Downloads][github-downloads-img]][github-downloads-uri]
[![Size][github-size-img]][github-size-img]
[![Last Commit][github-commit-img]][github-commit-img]
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
- [Tests](#tests)
- [Signed Releases](#signed-releases)
  - [Validate .Sig](#validate-sig)
  - [Validate SHA256SUM](#validate-sha256sum)
  - [Validate SHA1SUM](#validate-sha1sum)
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
| `Build & Release` | manual | same builds, then publishes a GitHub release with checksums. The version is read from `src-tauri/tauri.conf.json` and must match `src-tauri/Cargo.toml` |

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

# Signed Releases

These steps allow you to validate the files `sha256sum.txt.asc`, `sha1sum.txt.asc`, and `sha256sum.sig` which we provide with every release.

Validating these files helps ensure that the actual .zip or binary files you have downloaded indeed came from the real developer. Before you start reading the sections below; ensure you have the following packages installed:

- GPG
- sha1sum
- sha256sum

<br />

You can install these by running the commands:

```shell
# Ubuntu / Debian Systems
sudo apt update
sudo apt install gpg gnupg sha1sum sha256sum

# Fedora / Redhat
(yum | or | dnf ) install gpg gnupg
```

<br />

## Validate .Sig

Each release includes two `.sig` files; these are known as a GPG **Detached Signature**.

- sha1sum.sig
- sha256sum.sig

<br />

To validate that these signatures are good, download each of the files to your local machine:

```shell
# sha1sum
wget https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/sha1sum.sig

# sha256sum
wget https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/sha256sum.sig
```

<br />

Next, you will need the two associated files:

- sha1sum.txt.asc
- sha256sum.txt.asc

<br />

To download these, run the commands:

```shell
# sha1sum.txt.asc
wget https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/sha1sum.txt.asc

# sha256sum.txt.asc
wget https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/sha256sum.txt.asc
```

<br />

Now we can validate that these files indeed came from the original developer; run the command:

```shell
gpg --verify sha1sum.sig sha1sum.txt.asc
```

<br />

You should see the output:

```shell
gpg:                using RSA key 6921C2D3F6AE1188B7721C72F397BC89486A29A6
gpg: Good signature from "Binary Ninja (RSA 4096) <thebinaryninja@proton.me>" [unknown]
Primary key fingerprint: E8BA 8C94 B334 1673 0D8C  05C2 5769 7A1C BA63 B9FE
     Subkey fingerprint: 6921 C2D3 F6AE 1188 B772  1C72 F397 BC89 486A 29A6
```

<br />

Now validate the `sha256sum.sig` with the `sha256sum.txt.asc`:

```shell
gpg --verify sha256sum.sig sha256sum.txt.asc
```

<br />

You should see the output:

```shell
gpg:                using RSA key 6921C2D3F6AE1188B7721C72F397BC89486A29A6
gpg: Good signature from "Binary Ninja (RSA 4096) <thebinaryninja@proton.me>" [unknown]
Primary key fingerprint: E8BA 8C94 B334 1673 0D8C  05C2 5769 7A1C BA63 B9FE
     Subkey fingerprint: 6921 C2D3 F6AE 1188 B772  1C72 F397 BC89 486A 29A6
```

<br />

If you get the above results; you have validated the signature files.

<br />

<br />

## Validate SHA256SUM

This section explains how you can validate the downloaded `.zip` files and see if they are indeed authentic from the original developer. Before you can validate these files, you need to download the public GPG key we use when we sign releases. You can download it by running the following command:

```shell
curl -s https://github.com/BinaryServ.gpg | gpg --import
```

<br />

Download the `sha256sum.txt.asc` and validate the file itself:

```shell
wget https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/sha256sum.txt.asc

gpg --verify sha256sum.txt.asc
```

<br />

You should see the following. The value `6921C2D3F6AE1188B7721C72F397BC89486A29A6` is the key's fingerprint.

```shell
gpg: Signature made Mon 01 Jun 2025 00:00:00 UTC
gpg:                using RSA key 6921C2D3F6AE1188B7721C72F397BC89486A29A6
gpg: Good signature from "Binary Ninja (RSA 4096) <thebinaryninja@proton.me>" [ultimate]
```

<br />

You can view your existing GPG key fingerprints by running:

```shell
gpg --list-keys --keyid-format=long --fingerprint --with-fingerprint
```

<br />

This will allow you to see where the `692XXXXX` value comes from:

```shell
pub   rsa4096 2025-04-21 [C]
      E8BA 8C94 B334 1673 0D8C  05C2 5769 7A1C BA63 B9FE
uid           [ unknown] Binary Ninja (RSA 4096) <thebinaryninja@proton.me>
sub   rsa4096 2025-04-21 [S]
 >>>> 6921 C2D3 F6AE 1188 B772  1C72 F397 BC89 486A 29A6
sub   rsa4096 2025-04-21 [E]
      25E9 AFEC C12F 2072 AA6E  D5AB B077 63D0 A12F 0ED8
sub   rsa4096 2025-04-21 [A]
      AD79 A1EA CE17 962F B4ED  B30A DBA8 EA76 62FC 54E2
```

<br />

Next, download the `sha256sum.sig`

```shell
wget https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/sha256sum.sig
```

<br />

To validate the `.sig` file and your `sha256sum` file, run the command:

```shell
gpg --verify sha256sum.sig sha256sum.txt.asc
```

<br />

You should see:

```shell
gpg:                using RSA key 6921C2D3F6AE1188B7721C72F397BC89486A29A6
gpg: Good signature from "Binary Ninja (RSA 4096) <thebinaryninja@proton.me>" [unknown]
Primary key fingerprint: E8BA 8C94 B334 1673 0D8C  05C2 5769 7A1C BA63 B9FE
     Subkey fingerprint: 6921 C2D3 F6AE 1188 B772  1C72 F397 BC89 486A 29A6
```

<br />

You can now validate the `sha256sum.txt.asc` file against the files you have downloaded from our repository. In order to verify that the files are legit, you must download them from the [Releases](https://github.com/Aetherinox/ntfy-desktop/releases) page. They end with the extension `.zip`. You can also download them using `wget`. Make sure you download the zip files to the same folder where you downloaded the `sha256sum.txt.asc` and `sha256sum.sig`

```shell
# Download ntfy-desktop-2.1.1-darwin-amd64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-darwin-amd64.zip

# Download ntfy-desktop-2.1.1-darwin-arm64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-darwin-arm64.zip

# Download ntfy-desktop-2.1.1-linux-amd64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-linux-amd64.zip

# Download ntfy-desktop-2.1.1-linux-arm64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-linux-arm64.zip

# Download ntfy-desktop-2.1.1-linux-armv7l.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-linux-armv7l.zip

# Download ntfy-desktop-2.1.1-win32-amd64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-win32-amd64.zip

# Download ntfy-desktop-2.1.1-win32-arm64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-win32-arm64.zip

# Download ntfy-desktop-2.1.1-win32-ia32.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-win32-ia32.zip
```

<br />

Next, extract the hashes from the hash digest `sha256sum.txt.asc` file by running the command:

```shell
gpg --output sha256sum.txt --verify sha256sum.txt.asc
```

<br />

The command above will create a new file called `sha256sum.txt`, now run that file with the program `sha256sum`:

```shell
sha256sum --check sha256sum.txt
```

<br />

Which will output:

```shell
# Successful validation
ntfy-desktop-2.1.1-win32-ia32.zip: OK
ntfy-desktop-2.1.1-darwin-amd64.zip: OK
ntfy-desktop-2.1.1-win32-amd64.zip: OK
ntfy-desktop-2.1.1-win32-arm64.zip: OK
ntfy-desktop-2.1.1-darwin-arm64.zip: OK
ntfy-desktop-2.1.1-linux-armv7l.zip: OK
ntfy-desktop-2.1.1-linux-amd64.zip: OK
ntfy-desktop-2.1.1-linux-arm64.zip: OK

# Files missing for validation
ntfy-desktop-2.1.1-linux-arm64.zip: FAILED open or read

# File did not pass validation
ntfy-desktop-2.1.1-linux-arm64.zip: FAILED
```

<br />

If you see `FAILED` and the files are indeed there, ensure you downloaded them from our repository and not somewhere else.

<br />

If you see `OK`; this means the files are valid.

<br />

## Validate SHA1SUM

This section explains how you can validate the downloaded `.zip` files and see if they are indeed authentic from the original developer. Validating the `sha1sum.txt.asc` is similar to the `sha256sum.txt.asc`, first, download the public GPG key we use when we sign releases. You can download it by running the following command:

```shell
curl -s https://github.com/BinaryServ.gpg | gpg --import
```

<br />

Next, download the `sha1sum.txt.asc` file:

```shell
wget https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/sha1sum.txt.asc
```

<br />

Now verify that the `sha1sum.txt.asc` is properly signed:

```shell
gpg --verify sha1sum.txt.asc
```

<br />

You should see the following. The value `6921C2D3F6AE1188B7721C72F397BC89486A29A6` is the key's fingerprint.

```shell
gpg: Signature made Mon 01 Jun 2025 00:00:00 UTC
gpg:                using RSA key 6921C2D3F6AE1188B7721C72F397BC89486A29A6
gpg: Good signature from "Binary Ninja (RSA 4096) <thebinaryninja@proton.me>" [ultimate]
```

<br />

You can view your existing GPG key fingerprints by running:

```shell
gpg --list-keys --keyid-format=long --fingerprint --with-fingerprint
```

<br />

This will allow you to see where the `692XXXXX` value comes from:

```shell
pub   rsa4096 2025-04-21 [C]
      E8BA 8C94 B334 1673 0D8C  05C2 5769 7A1C BA63 B9FE
uid           [ unknown] Binary Ninja (RSA 4096) <thebinaryninja@proton.me>
sub   rsa4096 2025-04-21 [S]
 >>>> 6921 C2D3 F6AE 1188 B772  1C72 F397 BC89 486A 29A6
sub   rsa4096 2025-04-21 [E]
      25E9 AFEC C12F 2072 AA6E  D5AB B077 63D0 A12F 0ED8
sub   rsa4096 2025-04-21 [A]
      AD79 A1EA CE17 962F B4ED  B30A DBA8 EA76 62FC 54E2
```

<br />

Next, download the `sha1sum.sig`

```shell
wget https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/sha1sum.sig
```

<br />

To validate the `sha1sum.sig` file and your `sha1sum.txt.asc` file, run the command:

```shell
gpg --verify sha1sum.sig sha1sum.txt.asc
```

<br />

You should see:

```shell
gpg:                using RSA key 6921C2D3F6AE1188B7721C72F397BC89486A29A6
gpg: Good signature from "Binary Ninja (RSA 4096) <thebinaryninja@proton.me>" [unknown]
Primary key fingerprint: E8BA 8C94 B334 1673 0D8C  05C2 5769 7A1C BA63 B9FE
     Subkey fingerprint: 6921 C2D3 F6AE 1188 B772  1C72 F397 BC89 486A 29A6
```

<br />

You can now validate the `sha1sum.txt.asc` file against the files you have downloaded from our repository. In order to verify that the files are legit, you must download them from the [Releases](https://github.com/Aetherinox/ntfy-desktop/releases) page. They end with the extension `.zip`. You can also download them using `wget`. Make sure you download the zip files to the same folder where you downloaded the `sha1sum.txt.asc` and `sha1sum.sig`

```shell
# Download ntfy-desktop-2.1.1-darwin-amd64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-darwin-amd64.zip

# Download ntfy-desktop-2.1.1-darwin-arm64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-darwin-arm64.zip

# Download ntfy-desktop-2.1.1-linux-amd64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-linux-amd64.zip

# Download ntfy-desktop-2.1.1-linux-arm64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-linux-arm64.zip

# Download ntfy-desktop-2.1.1-linux-armv7l.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-linux-armv7l.zip

# Download ntfy-desktop-2.1.1-win32-amd64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-win32-amd64.zip

# Download ntfy-desktop-2.1.1-win32-arm64.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-win32-arm64.zip

# Download ntfy-desktop-2.1.1-win32-ia32.zip
wget \
  https://github.com/Aetherinox/ntfy-desktop/releases/download/2.1.1/ntfy-desktop-2.1.1-win32-ia32.zip
```

<br />

Next, extract the hashes from the hash digest `sha1sum.txt.asc` file by running the command:

```shell
gpg --output sha1sum.txt --verify sha1sum.txt.asc
```

<br />

The command above will create a new file called `sha1sum.txt`, now run that file with the program `sha1sum`:

```shell
sha1sum --check sha1sum.txt
```

<br />

Which will output:

```shell
# Successful validation
ntfy-desktop-2.1.1-win32-ia32.zip: OK
ntfy-desktop-2.1.1-darwin-amd64.zip: OK
ntfy-desktop-2.1.1-win32-amd64.zip: OK
ntfy-desktop-2.1.1-win32-arm64.zip: OK
ntfy-desktop-2.1.1-darwin-arm64.zip: OK
ntfy-desktop-2.1.1-linux-armv7l.zip: OK
ntfy-desktop-2.1.1-linux-amd64.zip: OK
ntfy-desktop-2.1.1-linux-arm64.zip: OK

# Files missing for validation
ntfy-desktop-2.1.1-linux-arm64.zip: FAILED open or read

# File did not pass validation
ntfy-desktop-2.1.1-linux-arm64.zip: FAILED
```

<br />

If you see `FAILED` and the files are indeed there, ensure you downloaded them from our repository and not somewhere else.

<br />

If you see `OK`; this means the files are valid.

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

- Review [active questions by our community](https://github.com/aetherinox/ntfy-desktop/labels/❔%20Question) and answer the ones you know.

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







<!-- BADGE > GENERAL -->
  [general-npmjs-uri]: https://npmjs.com
  [general-nodejs-uri]: https://nodejs.org
  [general-npmtrends-uri]: http://npmtrends.com/ntfy-desktop

<!-- BADGE > VERSION > GITHUB -->
  [github-version-img]: https://img.shields.io/github/v/tag/aetherinox/ntfy-desktop?logo=GitHub&label=Version&color=ba5225
  [github-version-uri]: https://github.com/aetherinox/ntfy-desktop/releases

<!-- BADGE > VERSION > NPMJS -->
  [npm-version-img]: https://img.shields.io/npm/v/ntfy-desktop?logo=npm&label=Version&color=ba5225
  [npm-version-uri]: https://npmjs.com/package/ntfy-desktop

<!-- BADGE > VERSION > PYPI -->
  [pypi-version-img]: https://img.shields.io/pypi/v/ntfy-desktop
  [pypi-version-uri]: https://pypi.org/project/ntfy-desktop/

<!-- BADGE > LICENSE > MIT -->
  [license-mit-img]: https://img.shields.io/badge/MIT-FFF?logo=creativecommons&logoColor=FFFFFF&label=License&color=9d29a0
  [license-mit-uri]: https://github.com/aetherinox/ntfy-desktop/blob/main/LICENSE

<!-- BADGE > GITHUB > DOWNLOAD COUNT -->
  [github-downloads-img]: https://img.shields.io/github/downloads/aetherinox/ntfy-desktop/total?logo=github&logoColor=FFFFFF&label=Downloads&color=376892
  [github-downloads-uri]: https://github.com/aetherinox/ntfy-desktop/releases

<!-- BADGE > NPMJS > DOWNLOAD COUNT -->
  [npmjs-downloads-img]: https://img.shields.io/npm/dw/%40aetherinox%2Fntfy-desktop?logo=npm&&label=Downloads&color=376892
  [npmjs-downloads-uri]: https://npmjs.com/package/ntfy-desktop

<!-- BADGE > GITHUB > DOWNLOAD SIZE -->
  [github-size-img]: https://img.shields.io/github/repo-size/aetherinox/ntfy-desktop?logo=github&label=Size&color=59702a
  [github-size-uri]: https://github.com/aetherinox/ntfy-desktop/releases

<!-- BADGE > NPMJS > DOWNLOAD SIZE -->
  [npmjs-size-img]: https://img.shields.io/npm/unpacked-size/ntfy-desktop/latest?logo=npm&label=Size&color=59702a
  [npmjs-size-uri]: https://npmjs.com/package/ntfy-desktop

<!-- BADGE > CODECOV > COVERAGE -->
  [codecov-coverage-img]: https://img.shields.io/codecov/c/github/aetherinox/ntfy-desktop?token=MPAVASGIOG&logo=codecov&logoColor=FFFFFF&label=Coverage&color=354b9e
  [codecov-coverage-uri]: https://codecov.io/github/aetherinox/ntfy-desktop

<!-- BADGE > ALL CONTRIBUTORS -->
  [contribs-all-img]: https://img.shields.io/github/all-contributors/aetherinox/ntfy-desktop?logo=contributorcovenant&color=de1f6f&label=contributors
  [contribs-all-uri]: https://github.com/all-contributors/all-contributors

<!-- BADGE > GITHUB > BUILD > NPM -->
  [github-build-img]: https://img.shields.io/github/actions/workflow/status/aetherinox/ntfy-desktop/npm-build.yml?logo=github&logoColor=FFFFFF&label=Build&color=%23278b30
  [github-build-uri]: https://github.com/aetherinox/ntfy-desktop/actions/workflows/npm-build.yml

<!-- BADGE > GITHUB > BUILD > Pypi -->
  [github-build-pypi-img]: https://img.shields.io/github/actions/workflow/status/aetherinox/ntfy-desktop/app-pypi.yml?logo=github&logoColor=FFFFFF&label=Build&color=%23278b30
  [github-build-pypi-uri]: https://github.com/aetherinox/ntfy-desktop/actions/workflows/app-pypi.yml

<!-- BADGE > GITHUB > TESTS -->
  [github-tests-img]: https://img.shields.io/github/actions/workflow/status/aetherinox/ntfy-desktop/npm-tests.yml?logo=github&label=Tests&color=2c6488
  [github-tests-uri]: https://github.com/aetherinox/ntfy-desktop/actions/workflows/npm-tests.yml?logo=github&label=Tests&color=2c6488
  [github-tests-uri]: https://github.com/aetherinox/ntfy-desktop/actions/workflows/app-tests.yml.yml

<!-- BADGE > GITHUB > COMMIT -->
  [github-commit-img]: https://img.shields.io/github/last-commit/aetherinox/ntfy-desktop?logo=conventionalcommits&logoColor=FFFFFF&label=Last%20Commit&color=313131
  [github-commit-uri]: https://github.com/aetherinox/ntfy-desktop/commits/main/

<!-- prettier-ignore-end -->
<!-- markdownlint-restore -->
