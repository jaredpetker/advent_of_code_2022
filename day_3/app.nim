import std/strutils, std/strformat

type
  Compartments = tuple[first: string, second: string]

# part 1
iterator readCompartments(filename: string): Compartments =
  for line in filename.lines:
    yield (line[0 .. int(line.len / 2) - 1], line[int(line.len / 2) .. line.len - 1])

proc getItemValue(item: char): Natural =
  let rawValue = ord(item)
  if rawValue >= 97:
    return rawValue - 96
  else:
    return rawValue - 38

proc getDuplicatedItemPrioritySum(filename: string): Natural =
  var sum = 0
  for compartments in readCompartments(filename):
    for c in compartments.first:
      if compartments.second.contains(c):
        sum += getItemValue(c)
        break
  return sum

# part 2
iterator readGroups(filename: string): tuple[a: string, b: string, c: string] =
  var grouped = newSeq[string]()
  for line in filename.lines:
    grouped.add(line)
    if grouped.len == 3:
      yield (grouped[0], grouped[1], grouped[2])
      grouped.setLen(0)

proc getGroupBadgePrioritySum(filename: string): Natural =
  var sum = 0
  for group in readGroups(filename):
    for c in group.a:
      if group.b.contains(c) and group.c.contains(c):
        sum += getItemValue(c)
        break
  return sum

# print answers
proc partOne(): Natural =
  return getDuplicatedItemPrioritySum("input.txt")

let partOneAnswer = partOne()
echo fmt"Part One: {partOneAnswer}"

proc partTwo(): Natural =
  return getGroupBadgePrioritySum("input.txt")

let partTwoAnswer = partTwo()
echo fmt"Part Two: {partTwoAnswer}"

