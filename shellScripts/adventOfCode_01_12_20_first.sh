### this is an attempt to solve an advent of code problem for 01.12.2020 at https://adventofcode.com/2020/day/1/
### Author: Sonke Wohler
### Date 11.12.2020

### this is the input file, put change this as you need
inputFile="/d/temp/input.txt"

### Step 1: 
### put input lines into buckets
### the more buckets the faster the next step would be
let i=0
let j=0
while read -r line
do
  if (($line < 1000)) 
  then
    lessThan[$i]=$line
    let i=i+1
  else
    moreThan[$j]=$line
    let j=j+1
  fi
done < "/d/temp/input.txt"

### Step 2:
### add the matching buckets together until you find the match
let lessSize=i-1
let moreSize=j-1
## let i=0
## let j=0

for lessI in $(seq 0 $lessSize)
do
  for moreJ in $(seq 0 $moreSize)
  do
    # some annoying issues with doing this in one line, so a temp variable makes it more readable and work in the first place
    let temp=${lessThan[$lessI]}+${moreThan[$moreJ]}
    if [ $temp = 2020 ]
    then
      break
    fi
  done
  if [ $temp = 2020 ]
  then
    break
  fi
done

### Step 3:
### output the results
echo the numbers are ${lessThan[$lessI]} and ${moreThan[$moreJ]} = $[${lessThan[$lessI]} + ${moreThan[$moreJ]}] 
echo the answer is $[${lessThan[$lessI]} * ${moreThan[$moreJ]}]

