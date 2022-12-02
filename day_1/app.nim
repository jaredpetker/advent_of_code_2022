import strutils, std/heapqueue, std/strformat

iterator readLine(filename: string): string =
  let file = open(filename)
  defer: file.close()
  let lines = readAll(file)
  for line in lines.split('\n'):
    yield line

proc part_one(): Natural =
  var sum = 0
  var max = 0
  for line in readLine("input.txt"):
    if line == "":
      if sum > max:
        max = sum
      sum = 0
    else:
      sum += parseInt(line)
  return max

let part_one_answer = part_one()
echo fmt"Part One: {part_one_answer}"

proc part_two(): Natural =
  var top_n = initHeapQueue[Natural]()
  var sum = 0
  for line in readLine("input.txt"):
    if line == "":
      let l = len(top_n)
      if l == 0:
        top_n.push(sum)
      elif len(top_n) > 0 and top_n[0] < sum:
        if len(top_n) == 3:
          discard replace(top_n, sum)
        else:
          top_n.push(sum)
      sum = 0
    else:
      sum += parseInt(line)
  sum = 0
  while len(top_n) > 0:
    sum += pop(top_n)
  return sum

let part_two_answer = part_two()
echo fmt"Part Two: {part_two_answer}"
