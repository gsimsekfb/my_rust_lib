@REM Usage: ta
@REM Usage: dir C:\Aliases  // Hint: We created this folder
@REM https://stackoverflow.com/questions/20530996/aliases-in-windows-command-prompt
@REM Create a folder called C:\Aliases
@REM Add C:\Aliases to your path (so any files in it will be found every time)
@REM Create a .bat file in C:\Aliases for each of the aliases you want
@REM e.g.
@REM The content of C:\Aliases\tp.bat :

@echo off
echo.
cargo test --all %*