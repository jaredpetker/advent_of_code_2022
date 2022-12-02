import strutils, std/heapqueue, std/strformat

iterator readLine(filename: string): string =
  let file = open(filename)
  defer: file.close()
  let lines = readAll(file)
  for line in lines.split('\n'):
    yield line

iterator readChunk(filename: string): Natural =
  var sum = 0
  for line in readLine(filename):
    if line == "":
      let res = sum
      sum = 0
      yield res
    else:
      sum += parseInt(line)

proc topNChunks(filename: string, n: Natural): Natural =
  var topN = initHeapQueue[Natural]()
  for sum in readChunk(filename):
    let l = len(topN)
    if l == 0:
      topN.push(sum)
    elif len(topN) > 0 and topN[0] < sum:
      if len(topN) == n:
        discard replace(topN, sum)
      else:
        topN.push(sum)
  var sum = 0
  while len(topN) > 0:
    sum += pop(topN)
  return sum

proc partOne(): Natural =
  return topNChunks("input.txt", 1)

let partOneAnswer = partOne()
echo fmt"Part One: {partOneAnswer}"

proc partTwo(): Natural =
  return topNChunks("input.txt", 3)

let partTwoAnswer = partTwo()
echo fmt"Part Two: {partTwoAnswer}"
