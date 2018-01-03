cls

set RUST_BACKTRACE=1

IF EXIST "C:\Secrets\Conduit.toml" ( COPY C:\Secrets\Conduit.toml .\ )
IF EXIST "E:\Secrets\Conduit.toml" ( COPY E:\Secrets\Conduit.toml .\ )
cargo build  --features orm
if errorlevel 1 (
  exit /b %errorlevel%
)
cargo build  --features tiberius
if errorlevel 1 (
  exit /b %errorlevel%
)
start /B run.cmd
cargo test
if errorlevel 1 (
  taskkill /F /IM server.exe
  exit /b %errorlevel%
)
taskkill /F /IM server.exe
