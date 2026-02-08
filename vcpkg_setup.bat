@echo off
setlocal enabledelayedexpansion

REM Exit immediately if any command fails
set ERRORLEVEL=

REM Check if cargo-vcpkg is installed
where cargo-vcpkg >nul 2>&1
if errorlevel 1 (
    echo cargo-vcpkg not found. Installing...
    cargo install cargo-vcpkg
    if errorlevel 1 exit /b 1
)

REM Change to engine directory
cd engine || exit /b 1

REM Run cargo vcpkg build
cargo vcpkg -v build
if errorlevel 1 exit /b 1

endlocal
