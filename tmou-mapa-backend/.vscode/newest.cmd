@echo off
for /f %%i in ('dir %1\*-*.exe /b /s /od') do set LAST=%%i
%LAST%