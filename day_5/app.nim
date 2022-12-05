import std/strutils, std/re, std/sequtils as su, std/sugar, std/enumerate, std/algorithm

type MethodFn = proc (stacks: var seq[seq[string]], count: int, fromStack: int, toStack: int)

proc moveSingly(stacks: var seq[seq[string]], count: int, fromStack: int, toStack: int) =
  for i in 0..<count:
    let pos = stacks[fromStack].len - 1
    stacks[toStack].add(stacks[fromStack][pos])
    stacks[fromStack].delete(pos..pos)

proc moveChunk(stacks: var seq[seq[string]], count: int, fromStack: int, toStack: int) =
    stacks[toStack].add(stacks[fromStack][^count..^1])
    stacks[fromStack].delete(stacks[fromStack].len - count .. stacks[fromStack].len - 1)

proc rearrangeStacks(filename: string, methodFn: MethodFn): string =
  var lStackAxis = -1
  var lines = newSeq[string]()
  var stacks: seq[seq[string]]

  for i, line in enumerate(filename.lines):
    if match(line, re"(\s\d+\s)+"):
      let s = findAll(line, re"(\s\d+\s)+")
      lStackAxis = i
      stacks = newSeq[seq[string]](s.len)

      for i in 0..<s.len:
        stacks[i] = newSeq[string]()
      lines.reverse()

      for stackLine in lines:
        var matches = newSeq[string](s.len)
        let regex = "^" & join(su.repeat("[\\[\\s](\\w+|\\s)[\\]\\s]", s.len), "\\s") & "$"
        discard match(stackLine, re(regex), matches)
        for idx, box in enumerate(matches):
          if box != " ":
            stacks[idx].add(box)

    elif lStackAxis != -1 and i > lStackAxis + 1:
      var commands = 0
      var matches = newSeq[string](3)
      discard match(line, re"move (\d+) from (\d+) to (\d+)", matches)
      let count = parseInt(matches[0])
      let fromStack = parseInt(matches[1]) - 1
      let toStack = parseInt(matches[2]) - 1

      methodFn(stacks, count, fromStack, toStack)

    else:
      lines.add(line)
  return stacks.map(stack => stack[stack.len - 1]).join("")

proc part1: string = return rearrangeStacks("input.txt", moveSingly)
proc part2: string = return rearrangeStacks("input.txt", moveChunk)

echo part1()
echo part2()
