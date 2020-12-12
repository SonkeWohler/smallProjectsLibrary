### this is an attempt to solve an advent of code problem for 01.12.2020 at https://adventofcode.com/2020/day/1#part2
### Author: Sonke Wohler
### Date 12.12.2020

### this is the input file, change this as you need
inputFile="/d/temp/input.txt"

### Step 1: 
### put input lines into buckets
### the more buckets the faster the next step would be
### turns out the most efficient buckets are probably the units of each number

# associative arrays are the closest to multidimensional ones that bash has: https://stackoverflow.com/questions/16487258/how-to-declare-2d-array-in-bash
declare -A bucket 
for counter in $(seq 0 9)
do
  let i[$counter]=0
done

while read -r line
do
  let unit=$line%10
  # keep expansions like this to separate lines to prevent unintended side effects, even if I find it detremental for readability
  let counter=${i[$unit]} 
  let bucket[$unit,$counter]=$line
  let i[$unit]+=1
done < $inputFile

for counter in $(seq 0 9)
do
  let size[$counter]=${i[$counter]}
done


### Step 2:
### add the matching buckets together until you find the match
# debugging
for counter in $(seq 0 9)
do
  for ((j=0;j<${size[$counter]};j++))
  do
    let tempArray[$j]=${bucket[$counter,$j]}
  done
  echo ${tempArray[*]}
  unset tempArray
done
echo
echo
echo done
