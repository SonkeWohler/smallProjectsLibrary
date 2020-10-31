@echo off
REM allows you to track the number of hours you have worked on a particular day

set todayDate=%date%
set todayTime=0
:start-question
echo -----
echo start working?
PAUSE
:stop-question
REM for loop copied from https://stackoverflow.com/questions/9922498/calculate-time-difference-in-windows-batch-file
for /F "tokens=1-4 delims=:.," %%a in ("%time%") do (set /A "startTime=(((%%a*60)+1%%b %% 100)*60+1%%c %% 100)*100+1%%d %% 100")
PAUSE
:calculate
for /F "tokens=1-4 delims=:.," %%a in ("%time%") do (set /A "endTime=(((%%a*60)+1%%b %% 100)*60+1%%c %% 100)*100+1%%d %% 100")
set /a todayTime=todayTime+endTime/100-startTime/100
echo You have worked %todayTime% seconds since starting this program
set /a todayHours=%todayTime% / 3600
set /a todaySeconds=%todayTime% %% 3600
set /a todayMinutes=%todaySeconds% / 60
set /a todaySeconds=%todaySeconds% %% 60
echo The time you have worked today is %todayHours%:%todayMinutes%:%todaySeconds%
goto :start-question
