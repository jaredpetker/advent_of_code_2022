import java.io.File
import java.util.stream.IntStream

data class Dir(val dx: IntProgression, val dy: IntProgression)



data class Pos(val x: Int, val y: Int)

data class GridItem(val item: Int, val pos: Pos)

class Grid constructor(private val grid: List<List<Int>>)  {
    companion object {}

    private val width = grid[0].size
    private val height = grid.size
    val visibleTreeCount: Int = countVisibleTrees()
    val highestScenicScore: Int = findHighestScenicScore()

    private fun sequence(): Sequence<GridItem> = sequence {
        grid.forEachIndexed {y, row ->
            row.forEachIndexed {x, item ->
                yield(GridItem(item, Pos(x, y)))
            }
        }
    }

    private fun directions(x: Int, y: Int, width: Int, height: Int): Sequence<Dir> = sequence {
        yield(Dir(x - 1 downTo 0, y .. y))
        yield(Dir(x + 1 until  width, y .. y))
        yield(Dir(x .. x, y - 1 downTo 0))
        yield(Dir(x .. x, y + 1 until height))
    }

    private fun countVisibleTrees(): Int {
        return sequence().mapNotNullTo(HashSet()) mapTo@ { (item, pos) ->
            dirIter@ for ((dx, dy) in directions(pos.x, pos.y, width, height)) {
                for (px in dx) {
                    for (py in dy) {
                        if (grid[py][px] >= item) {
                            continue@dirIter
                        }
                    }
                }
                return@mapTo pos.y * width + pos.x
            }
            return@mapTo null
        }.size
    }

    private fun findHighestScenicScore(): Int {
        return sequence().fold(0) fold@ { acc, (item, pos) ->
            val score = directions(pos.x, pos.y, width, height).fold(1) { scoreAcc, (dx, dy) ->
                var treeCount = 0
                loop@ for (px in dx) {
                    for (py in dy) {
                        val tHeight = grid[py][px]
                        if (tHeight <= item) {
                            treeCount++
                        }
                        if (tHeight >= item) {
                            break@loop
                        }
                    }
                }
                scoreAcc * treeCount
            }
            return@fold if (score > acc) score else acc
        }
    }
}

fun Grid.Companion.parse(input: List<String>): Grid {
    return Grid(input.map { str -> str.toList().map { it.digitToInt() } })
}

fun main() {
    val input = File("input.txt").readLines()
    println(Grid.parse(input).visibleTreeCount)
    println(Grid.parse(input).highestScenicScore)
}
