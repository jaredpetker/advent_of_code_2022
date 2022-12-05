import std/strutils, std/re, std/sequtils, std/sugar


proc contained(nums: seq): bool =
  return ((nums[0] <= nums[2] and nums[1] >= nums[3]) or (nums[0] >= nums[2] and nums[1] <= nums[3]))

proc overlap(nums: seq): bool =
  return contained(nums) or
      ((nums[0] >= nums[2] and nums[0] <= nums[3]) or (nums[1] >= nums[2] and nums[1] <= nums[3]))

proc readLinesToSeq(filename: string): seq[seq[int]] =
  return toSeq(filename.lines).map(proc (line: string): seq[int] =
    var matches = newSeq[string](4)
    discard match(line, re"(\d+)-(\d+),(\d+)-(\d+)", matches)
    return matches.map(it => parseInt(it))
  )

proc part1(filename: string): Natural =
  return readLinesToSeq(filename).foldl(a + int(contained(b)), 0)


proc part2(filename: string): Natural =
  return readLinesToSeq(filename).foldl(a + int(overlap(b)), 0)

echo part1("input.txt")
echo part2("input.txt")
