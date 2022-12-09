import java.io.File
import kotlin.collections.ArrayList
import kotlin.math.absoluteValue

data class Point2D(var x: Int = 0, var y: Int = 0)
typealias Pos = Point2D
typealias Move = Point2D

enum class Move2D(val move: Move) {
    L(Move(-1, 0)),
    R(Move(1, 0)),
    U(Move(0, 1)),
    D(Move(0, -1))
}

fun moves(lines: List<String>) = sequence {
    var listPos = 0
    var currMove: Move2D = Move2D.U
    var moveCount = 0

    while (listPos < lines.size || moveCount > 0) {
       if (moveCount > 0) {
           yield(currMove)
           moveCount -= 1
       } else {
           val line = lines[listPos]
           val dir = line.take(1)
           val count = line.drop(2).toInt()
           currMove = Move2D.valueOf(dir)
           moveCount = count
           listPos += 1
       }
    }
}

class Grid constructor(private val knotCount: Int = 2) {
    private val knots: ArrayList<Pos> = (0 until knotCount).mapTo(ArrayList()) { Pos() }
    private val tailVisited = hashSetOf(Pos())

    private fun moveKnot(index: Int, move: Move) {
        val head = knots[index]
        head.x += move.x
        head.y += move.y
    }

    private fun settle() {
        for (i in 1 until knotCount) {
            val prev = knots[i - 1]
            val curr = knots[i]
            val dx = prev.x - curr.x
            val dy = prev.y - curr.y
            val isHeadFarDiagonally = (dx.absoluteValue + dy.absoluteValue) > 2
            moveKnot(i, Move(
                if (dx.absoluteValue > 1 || isHeadFarDiagonally) dx.coerceIn(-1, 1) else 0,
                if (dy.absoluteValue > 1 || isHeadFarDiagonally) dy.coerceIn(-1, 1) else 0
            ))
        }
        tailVisited.add(knots.last().copy())
    }

    fun step(move2D: Move2D)  {
        moveKnot(0, move2D.move)
        settle()
    }

    fun getTailVisitedCount() = tailVisited.size
}

fun main() {
    val lines = File("input.txt").readLines()
    val grid2Knots = Grid(2)
    val grid10Knots = Grid(10)
    for (move in moves(lines)) {
        grid2Knots.step(move)
        grid10Knots.step(move)
    }
    println("part 1 answer: ${grid2Knots.getTailVisitedCount()}")
    println("part 2 answer: ${grid10Knots.getTailVisitedCount()}")
}
