import std/strutils, std/strformat, std/sequtils, std/tables

type
  TheirMove {.pure.} = enum
    Rock = 'A', Paper = 'B', Scissors = 'C'
  MyMove {.pure.} = enum
      Rock = 'X', Paper = 'Y', Scissors = 'Z'
  RequiredMove {.pure.} = enum
      Lose = 'X', Draw = 'Y', Win = 'Z'
  RoundMoves = tuple[theirs: TheirMove, mine: MyMove]
  RequiredRoundMoves = tuple[theirs: TheirMove, required: RequiredMove]
  OutcomeScore {.pure.} = enum
      Lose = 0, Draw = 3, Win = 6

let MoveScore = {MyMove.Rock: 1, MyMove.Paper: 2, MyMove.Scissors: 3}.toTable
# way too complex but I wanted to play with nim's tuples and tables
let RoundScore = {
  (TheirMove.Scissors, MyMove.Rock): MoveScore[MyMove.Rock] + int(OutcomeScore.Win),
  (TheirMove.Paper, MyMove.Rock): MoveScore[MyMove.Rock] + int(OutcomeScore.Lose),
  (TheirMove.Rock, MyMove.Rock): MoveScore[MyMove.Rock] + int(OutcomeScore.Draw),

  (TheirMove.Scissors, MyMove.Paper): MoveScore[MyMove.Paper] + int(OutcomeScore.Lose),
  (TheirMove.Paper, MyMove.Paper): MoveScore[MyMove.Paper] + int(OutcomeScore.Draw),
  (TheirMove.Rock, MyMove.Paper): MoveScore[MyMove.Paper] + int(OutcomeScore.Win),

  (TheirMove.Scissors, MyMove.Scissors): MoveScore[MyMove.Scissors] + int(OutcomeScore.Draw),
  (TheirMove.Paper, MyMove.Scissors): MoveScore[MyMove.Scissors] + int(OutcomeScore.Win),
  (TheirMove.Rock, MyMove.Scissors): MoveScore[MyMove.Scissors] + int(OutcomeScore.Lose),
}.toTable

let RequiredMoveMap = {
  (TheirMove.Scissors, RequiredMove.Lose): MyMove.Paper,
  (TheirMove.Scissors, RequiredMove.Draw): MyMove.Scissors,
  (TheirMove.Scissors, RequiredMove.Win): MyMove.Rock,

  (TheirMove.Rock, RequiredMove.Lose): MyMove.Scissors,
  (TheirMove.Rock, RequiredMove.Draw): MyMove.Rock,
  (TheirMove.Rock, RequiredMove.Win): MyMove.Paper,

  (TheirMove.Paper, RequiredMove.Lose): MyMove.Rock,
  (TheirMove.Paper, RequiredMove.Draw): MyMove.Paper,
  (TheirMove.Paper, RequiredMove.Win): MyMove.Scissors,

}.toTable

# abstracted iterator for reading pairs of characters from the given input
iterator readPairs(filename: string): tuple[a: char, b: char] =
  let chunk = newSeq[Natural]()
  for line in filename.lines:
    let pair = line.split(' ')
    yield (pair[0][0], pair[1][0])

# for part 1
iterator readMoves(filename: string): RoundMoves =
  for pair in readPairs(filename):
    yield (TheirMove(pair.a), MyMove(pair.b))

proc processMoves(filename: string): Natural =
  return toSeq(readMoves(filename)).mapIt(RoundScore[it]).foldl(a + b)

# for part 2
# we need to treat the second item in the pair / tuple as the "required move"
# then translate to what our move should be
proc getRoundMoves(moves: RequiredRoundMoves): RoundMoves =
  return (moves.theirs, RequiredMoveMap[(moves.theirs, moves.required)])

iterator readMovesRequired(filename: string): RequiredRoundMoves =
  for pair in readPairs(filename):
    yield (TheirMove(pair.a), RequiredMove(pair.b))

proc processAndTranslateMoves(filename: string): Natural =
  return toSeq(readMovesRequired(filename)).mapIt(RoundScore[getRoundMoves(it)]).foldl(a + b)


# print answers
proc partOne(): Natural =
  return processMoves("input.txt")

let partOneAnswer = partOne()
echo fmt"Part One: {partOneAnswer}"

proc partTwo(): Natural =
  return processAndTranslateMoves("input.txt")

let partTwoAnswer = partTwo()
echo fmt"Part Two: {partTwoAnswer}"

