@echo off

echo:
echo ############## Building Rust Library ##############
echo:

maturin build

echo:
echo ############## Installing Library Wheel ##############
echo:

pip install .\target\wheels\ChessProject-0.1.0-cp310-none-win_amd64.whl

echo:
echo ############## Done ##############
echo:
