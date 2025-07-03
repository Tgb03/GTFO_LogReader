
@echo off 
setlocal enabledelayedexpansion

for /f "tokens=2 delims== " %%v in ('findstr /r "^version" Cargo.toml') do (
	set "version=%%v"
)

@echo on
@echo Version is -%version%-
@echo off 

cargo build
cargo build --release
cargo build --target i686-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc

md target\git_release\%version%

copy target\release\gtfo_log_reader_core.dll target\git_release\%version%\gtfo_log_reader_core_64bit.dll
copy target\debug\gtfo_log_reader_core.dll target\git_release\%version%\gtfo_log_reader_core_64bit_debug.dll
copy target\i686-pc-windows-msvc\release\gtfo_log_reader_core.dll target\git_release\%version%\gtfo_log_reader_core_32bit.dll
copy target\i686-pc-windows-msvc\debug\gtfo_log_reader_core.dll target\git_release\%version%\gtfo_log_reader_core_32bit_debug.dll

copy resources\level_descriptors.json target\git_release\%version%\resources\level_descriptors.json

endlocal
