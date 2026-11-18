@echo off
set "NATIVE=%~dp0native\target\release\genesis.exe"
if not exist "%NATIVE%" (
    echo Genesis is not built. Run: cargo build --release --manifest-path "%~dp0native\Cargo.toml"
    exit /b 1
)
start "" "%NATIVE%"
