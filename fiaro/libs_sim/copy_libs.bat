@echo off
prompt $G 
cd %~dp0
@echo on

xcopy /Y *.a ..\target\debug\deps\
xcopy /Y *.dll ..\target\debug\