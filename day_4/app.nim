import std/strutils, std/re, std/sequtils


proc contained(nums: seq): bool =
  return ((nums[0] <= nums[2] and nums[1] >= nums[3]) or (nums[0] >= nums[2] and nums[1] <= nums[3]))

proc overlap(nums: seq): bool =
  return contained(nums) or
      ((nums[0] >= nums[2] and nums[0] <= nums[3]) or (nums[1] >= nums[2] and nums[1] <= nums[3]))

proc part1(filename: string): Natural =
  var count = 0
  for line in filename.lines:
    var matches = newSeq[string](4)
    discard match(line, re"(\d+)-(\d+),(\d+)-(\d+)", matches)
    let nums = matches.mapIt(parseInt(it))
    count += int(contained(nums))
  return count


proc part2(filename: string): Natural =
  var count = 0
  for line in filename.lines:
    var matches = newSeq[string](4)
    discard match(line, re"(\d+)-(\d+),(\d+)-(\d+)", matches)
    let nums = matches.mapIt(parseInt(it))
    count += int(overlap(nums))
  return count

echo part1("input.txt")
echo part2("input.txt")
