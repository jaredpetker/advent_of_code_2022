import java.io.File
import java.util.*

data class Quadruple<A,B,C,D>(var first: A, var second: B, var third: C, var fourth: D) {
    override fun toString(): String = "($first, $second, $third, $fourth)"
}

data class Piece(val pattern: String, val stride: Int) {
    val width = pattern.indexOf('\n').takeIf { it > -1 } ?: pattern.length
    val height = pattern.count { it == '\n' } + 1
    val bits = run {
        var mod = 0
        var j = 0
        pattern.toCharArray().withIndex().fold(0) { acc, indexed ->
            var b = acc
            val (i, c) = indexed
            if (c == '#') {
                b = b or ((1 shl ((i - j) + mod)))
            } else if (c == '\n') {
                j = i + 1
                mod += stride
            }
            b
        }
    }

    fun display() {
        print("\n")
        println("width: $width, height: $height")
        println(this)
    }

    override fun toString(): String {
        return Integer.toBinaryString(this.bits).reversed().chunked(7).joinToString("\n")
    }
}

fun Int.firstNBits(n: Int): String {
    val b = Integer.toBinaryString(this)
    return "0".repeat(n).replaceRange(n - b.length until n, b)
//  .replace("1", "#").replace("0", ".")
}

data class Chamber(val width: Int = 7) {
    private val jet = File("input.txt").readText()
    private val pieces = listOf(
        Piece("####", 7),
        Piece(
            """
            .#.
            ###
            .#.
        """.trimIndent(), 7
        ),
        Piece(
            """
            ..#
            ..#
            ###
        """.trimIndent(), 7
        ),
        Piece(
            """
            #
            #
            #
            #
        """.trimIndent(), 7
        ),
        Piece(
            """
            ##
            ##
        """.trimIndent(), 7
        )
    )
    private var jetPos = 0
    private val grid: Vector<Int> = Vector()
    private var topMost = -1

    private val cache: HashMap<Quadruple<String, Boolean, Int, Int>, Pair<Long, Int>> = HashMap()
    private val topMostCache: HashMap<Long, Int> = HashMap()

    private fun dropPiece(piece: Piece, count: Long): Pair<Long, Long>? {

        // generate the piece rows
        val pieceRows = Array(piece.height) { 0 }
        var shiftedBy = 2
        for (h in 0 until piece.height) {
            val b = ((piece.bits and (0b1111111 shl h * width)) shr (h * width)) shl shiftedBy
            pieceRows[h] = b
        }

        // add height to the grid, set to 0
        grid.setSize(topMost + 4 + piece.height)
        for (i in topMost + 1 until grid.size) {
            if (grid[i] == null) {
                grid[i] = 0
            }
        }

        // current position of the bottom of the piece
        var curPosBot = grid.size - piece.height
        loop@ while (curPosBot >= 0) {

            // collision check
            for ((i, _) in pieceRows.withIndex()) {
                if (pieceRows[pieceRows.size - 1 - i] and grid[curPosBot + i] != 0) {
                    break@loop
                }
            }

            // jets sequencing
            if (jetPos >= jet.length) {
                jetPos = 0
            }

            // apply jet shift
            var dir = 0
            when (jet[jetPos]) {
                '<' -> {
                    // if there is room to shift over
                    if (shiftedBy > 0) {
                        // shift ever row over 1
                        dir = -1
                        shiftedBy += dir
                        for (i in pieceRows.indices) {
                            pieceRows[i] = pieceRows[i] shr 1
                        }

                    }
                }
                '>' -> {
                    // if there is room to shift over
                    if (width - piece.width > shiftedBy) {
                        // shift ever row over 1
                        dir = 1
                        shiftedBy += dir
                        for (i in pieceRows.indices) {
                            pieceRows[i] = pieceRows[i] shl 1
                        }

                    }
                }
            }

            // check collisions, rollback if needed
            if (curPosBot <= topMost) {
                for (i in pieceRows.indices) {
                    if (pieceRows[pieceRows.size - 1 - i] and grid[curPosBot + i] != 0) {
                        for (pi in pieceRows.indices) {
                            if (dir == 1) {
                                pieceRows[pi] = pieceRows[pi] shr 1
                            } else {
                                pieceRows[pi] = pieceRows[pi] shl 1
                            }
                        }
                        shiftedBy -= dir
                        break
                    }
                }
            }

            jetPos += 1
            curPosBot -= 1
        }
        topMost = topMost.coerceAtLeast(curPosBot + piece.height)

        for (i in 0 until piece.height) {
            grid[curPosBot + piece.height - i] = grid[curPosBot + piece.height - i] or pieceRows[i]
        }

        var blocked = false
        var rowAcc = 0
        val windowKey = StringBuilder()
        if (grid.size >= 4 && curPosBot >= 0) {
            for (i in 0 until 4) {
                rowAcc = rowAcc or grid[curPosBot + 3 - i]
                windowKey.append(rowAcc)
                if (rowAcc == 0b1111111) {
                    blocked = true
                }
            }
        }

        val cacheKey = Quadruple(windowKey.toString(), blocked, piece.bits, jetPos)
        topMostCache[count] = topMost
        return if (cache.containsKey(cacheKey) && blocked) {
            Pair(cache[cacheKey]!!.first, count)
        } else {
            cache[cacheKey] = Pair(count, topMost)
            null
        }
    }

    private fun nextPiece() = sequence {
        generateSequence(0) { it + 1 }.forEach {
            this.yield(pieces[it % pieces.size])
        }
    }

    fun run(max: Long): Long {
        val iter = nextPiece().iterator()
        var repeated: Pair<Long, Long>? = null
        for (i in 0 until max) {
            repeated = dropPiece(iter.next(), (i + 1))
            if (repeated != null) {
                break
            }
        }
        return if (repeated != null) {
            val div = (max - repeated.first) / (repeated.second - repeated.first)
            val rem = (max - repeated.first) % (repeated.second - repeated.first)
            topMostCache[repeated.first]!! + 1 +
                    (div * (topMostCache[repeated.second]!! - topMostCache[repeated.first]!!)) +
                    (topMostCache[repeated.first + rem]!! - topMostCache[repeated.first]!!)
        } else {
            (topMost + 1).toLong()
        }

    }
}

fun main() {
    println("Part 1 answer: ${Chamber().run(2022)}")
    println("Part 2 answer: ${Chamber().run(1000000000000)}")
}
