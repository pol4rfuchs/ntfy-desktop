@ECHO OFF
SETLOCAL
TITLE ntfy-desktop Build Script (Tauri)

:: #
::  Ntfy Desktop build script (Tauri / Rust)
::
::  builds the installers for Windows (NSIS .exe and .msi).
::
::  needs:  Rust (https://rustup.rs) with the MSVC toolchain, and the
::          "Desktop development with C++" part of the Visual Studio Build Tools.
::
::  usage:  build.bat               run tests, then build installers
::          build.bat --debug       extra arguments are passed to `cargo tauri build`
::
::  the previous Electron build lives in build-electron.bat
:: #

CD /d "%~dp0src-tauri"

echo.
echo =============================================================================
echo                         NTFY DESKTOP BUILD SCRIPT
echo =============================================================================
echo.

where cargo >nul 2>&1
IF ERRORLEVEL 1 (
    echo [ERROR] Rust ^(cargo^) not found. Install it from https://rustup.rs
    EXIT /B 1
)

cargo tauri --version >nul 2>&1
IF ERRORLEVEL 1 (
    echo [INSTALL] Tauri CLI not found, installing ^(one time, takes a few minutes^)...
    cargo install tauri-cli --version "^2" --locked || EXIT /B 1
)

echo [TEST] Running unit tests...
cargo test --locked || EXIT /B 1

echo [BUILD] Building installers...
cargo tauri build %* || EXIT /B 1

echo.
echo [OK] Done. Installers are in: %~dp0src-tauri\target\release\bundle
ENDLOCAL
