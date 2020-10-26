color 0a
@echo off
echo.
REM tutorials used:
REM		https://www.tutorialspoint.com/batch_script/index.htm
REM 	http://steve-jansen.github.io/guides/windows-batch-scripting/part-1-getting-started.html
REM using goto instead of for-loops for the fun of it.

:init
set length=10 
set changingDepth=7
set /a i=0
:init-if-statement
set /a rand=%random% %% 10
set list[%i%]=%rand%
set /a i+=1
if %i% lss %length% goto :init-if-statement

:run
set i=1
set list=%list[0]%
:run-loop-1
REM delayed expansion is all that's needed from https://stackoverflow.com/questions/20385885/echo-batch-file-arrays-using-a-variable-for-the-index
setlocal enableDelayedExpansion
set rand=!list[%i%]!
REM local/global hack from https://stackoverflow.com/questions/15494688/batch-script-make-setlocal-variable-accessed-by-other-batch-files
endlocal & (set rand=%rand%)
set list=%list% %rand%
set /a i+=1
if %i% lss %length% goto :run-loop-1
set /a i=%random% %% %changingDepth%
:run-loop-2
echo %list%
set /a i-=1
if %i% leq 0 goto :change
goto :run-loop-2

:change
set /a index=%random% %% %length%
set /a rand=%random% %% 10
set list[%index%]=%rand%
goto :run

:end
echo debugEnd



