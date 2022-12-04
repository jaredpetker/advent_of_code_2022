import std/strutils, std/strformat, std/sets

type
  Compartments = tuple[first: string, second: string]

proc getItemValue(item: char): Natural =
  let rawValue = ord(item)
  if rawValue >= 97:
    return rawValue - 96
  else:
    return rawValue - 38

# part 1
iterator readCompartments(filename: string): Compartments =
  for line in filename.lines:
    yield (line[0 .. int(line.len / 2) - 1], line[int(line.len / 2) .. line.len - 1])

proc getDuplicatedItemPrioritySum(filename: string): Natural =
  var sum = 0
  for compartments in readCompartments(filename):
    var c = toHashSet(compartments.first) * toHashSet(compartments.second)
    sum += getItemValue(c.pop)
  return sum

# part 2
proc nextLine(filename: string): iterator(): string =
  return iterator(): string =
    for line in filename.lines:
      yield line

proc getGroupBadgePrioritySum(filename: string): Natural =
  let next = nextLine(filename)
  var sum = 0
  while true:
    let (a, b, c) = (toHashSet(next()), toHashSet(next()), toHashSet(next()))
    if finished(next):
      break
    var intersection = a * b * c
    sum += getItemValue(intersection.pop)
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

