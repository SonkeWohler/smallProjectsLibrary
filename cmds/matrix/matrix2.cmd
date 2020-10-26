color 0a
@echo off
echo.
REM tutorials used:
REM		https://www.tutorialspoint.com/batch_script/index.htm
REM 	http://steve-jansen.github.io/guides/windows-batch-scripting/part-1-getting-started.html
REM used to use goto for fun, but was slow so trying for loops

:init
REM these values should be changed as needed, but works on a HD screen with standard script size rather well
REM the length of the numbers displayed
set length=118
REM the maximum number of times list is echoed before a number is changed randomly
set changingDepth=3
:init-if-statement
for /l %%i in (0,1,%length%) do (
	set /a rand=%random% %% 10
	set list[%i%]=%rand%
)

REM delayed expansion is all that's needed from the first answer here: https://stackoverflow.com/questions/20385885/echo-batch-file-arrays-using-a-variable-for-the-index
setlocal enableDelayedExpansion
:run
set list=!list[0]!
:run-loop-1
for /l %%i in (1,1,%length%) do (
	set rand=!list[%%i]!
	REM using ! instead of % so variables are evaluated "as late as possible" i.e. at each loop iteration, as opposed to "as soon as possible" i.e. before the loop reusing the value each iteration
	set list=!list! !rand!
)
:run-loop-2
set /a rand=%random% %% %changingDepth%
for /l %%i in (%rand%,-1,0) do (
	echo %list%
)
:change
set /a index=%random% %% %length%
set /a rand=%random% %% 10
set list[%index%]=%rand%
goto :run

:not-needed-anymore
REM local/global hack from https://stackoverflow.com/questions/15494688/batch-script-make-setlocal-variable-accessed-by-other-batch-files
endlocal & (set list=%list%)

:end
echo debugEnd



