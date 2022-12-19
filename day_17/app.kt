import java.io.File
import java.util.*
import kotlin.collections.HashMap

data class Piece(val pattern: String, val stride: Int) {
    val width = pattern.indexOf('\n').takeIf { it > -1 } ?: pattern.length
    val height = pattern.count { it == '\n' } + 1
    val bits = run {
        var mod = 0
        var j = 0;
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
        return Integer.toBinaryString(this.bits.toInt()).reversed().chunked(7).joinToString("\n")
    }
}

fun Int.firstNBits(n: Int): String {
    val b = Integer.toBinaryString(this)
    return "0".repeat(n).replaceRange(n - b.length until n, b).replace("1", "#").replace("0", ".")
}

data class Chamber(val width: Int = 7) {
    val jet = File("input.txt").readText()
    var jetPos = 0
    val grid: Vector<Int> = Vector()
    var topMost = -1
    val pieces = listOf(
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
    val cache: HashMap<String, Pair<Long, Int>> = HashMap()
    val topMostCache: HashMap<Long, Int> = HashMap()

    private fun dropPiece(piece: Piece, count: Long): Pair<Long, Long>? {

        // generate the piece rows
        val pieceRows = Array(piece.height) { 0 }
        var shiftBy = 2
        for (h in 0 until piece.height) {
            val b = ((piece.bits and (0b1111111 shl h * width)) shr (h * width)) shl shiftBy
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

            // TODO: this area is gnarly. bitshifts rows, checks collisions, rolls back if needed
            if (jet[jetPos] == '<') {
                // shift right
                if (shiftBy > 0) {
                    shiftBy -= 1
                    for (i in 0 until pieceRows.size) {
                        pieceRows[i] = pieceRows[i] shr 1
                    }
                    if (curPosBot <= topMost) {
                        for ((i, _) in pieceRows.withIndex()) {
                            if (pieceRows[pieceRows.size - 1 - i] and grid[curPosBot + i] != 0) {
                                for (i in 0 until pieceRows.size) {
                                    pieceRows[i] = pieceRows[i] shl 1
                                }
                                shiftBy += 1
                                break
                            }
                        }
                    }
                }

            } else {
                // shift left
                if (width - piece.width > shiftBy) {
                    shiftBy += 1
                    for (i in 0 until pieceRows.size) {
                        pieceRows[i] = pieceRows[i] shl 1
                    }
                    if (curPosBot <= topMost) {
                        for ((i, _) in pieceRows.withIndex()) {
                            if (pieceRows[pieceRows.size - 1 - i] and grid[curPosBot + i] != 0) {
                                for (i in 0 until pieceRows.size) {
                                    pieceRows[i] = pieceRows[i] shr 1
                                }
                                shiftBy -= 1
                                break
                            }
                        }
                    }
                }

            }

            jetPos += 1
            curPosBot -= 1
        }
        topMost = topMost.coerceAtLeast(curPosBot + piece.height)

        for (i in 0 until piece.height) {
            grid[curPosBot + piece.height - i] = grid[curPosBot + piece.height - i] or pieceRows[i]
//            if ((grid[curPosBot + piece.height - i] and 0b1111111) == 0b1111111) {
//                fullRow = curPosBot + piece.height - i
//            }
        }

        var fullRow = -1
        var runningOr = 0
        var windowKey = ""
        if (grid.size >= 4 && curPosBot >= 0) {
            for (i in 0 until 4) {
                runningOr = runningOr or grid[curPosBot + 3 - i]
                windowKey += runningOr
                if (runningOr == 0b1111111) {
                    fullRow = curPosBot
                }
            }
        }



        val cacheKey = "$windowKey/${fullRow >= 0}/$piece.bits/$jetPos"
        topMostCache[count] = topMost
        if (cache.containsKey(cacheKey) && fullRow >= 0) {
            return Pair(cache[cacheKey]!!.first, count)
        }
        cache[cacheKey] = Pair(count, topMost)
//        for (i in grid.reversed()) {
//            println(i.firstNBits(width).reversed())
//        }
        return null
    }

    private fun nextPiece() = sequence {
        generateSequence(0) { it + 1 }.forEach {
            this.yield(pieces[it % pieces.size])
        }
    }

    fun run(max: Long) {
        val iter = nextPiece().iterator()
        var repeated: Pair<Long, Long>? = null
        for (i in 0 until max) {
            repeated = dropPiece(iter.next(), (i + 1).toLong())
            if (repeated != null) {
                break
            }
        }
        if (repeated != null) {
            val div = (max - repeated.first) / (repeated.second - repeated.first)
            val rem = (max - repeated.first) % (repeated.second - repeated.first)
            println(
                topMostCache[repeated.first]!!
                        + (div * (topMostCache[repeated.second]!! - topMostCache[repeated.first]!!))
                        + (topMostCache[repeated.first + rem]!! - topMostCache[repeated.first]!!)
                        + 1
            )
        } else {
            println(topMost + 1)
        }

    }
}

fun main() {
    Chamber().run(2022)
    Chamber().run(1000000000000)
}
