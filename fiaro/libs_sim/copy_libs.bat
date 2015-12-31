@echo off
prompt $G 
cd %~dp0
@echo on

xcopy *.a ..\target\debug\deps\
xcopy *.dll ..\target\debug\