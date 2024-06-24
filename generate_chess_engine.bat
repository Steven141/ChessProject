@echo off

setlocal

echo:
echo ############## Building Rust Library ##############
echo:

:: Run maturin build and capture the output line by line to get wheel path
for /f "delims=" %%i in ('maturin build --release 2^>^&1') do (
    echo %%i
    set output=%%i
)

echo:
echo ############## Installing Library Wheel ##############
echo:

:: Extract the relative path from the wheel_path
for %%i in ("%output%") do set "relative_path=%%~pi"
set "relative_path=%relative_path:*\target=\target%"

:: Append the filename to the relative path
for %%i in ("%output%") do set "relative_path=%relative_path%%%~nxi"

pip install .%relative_path% --force-reinstall

echo:
echo ############## Done ##############
echo:

endlocal
