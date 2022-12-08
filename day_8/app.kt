import java.io.File

data class Dir(val dx: IntProgression, val dy: IntProgression)

fun getDirections(x: Int, y: Int, width: Int, height: Int): Array<Dir> {
    return arrayOf(
        Dir(x - 1 downTo 0, y .. y),
        Dir(x + 1 until  width, y .. y),
        Dir(x .. x, y - 1 downTo 0),
        Dir(x .. x, y + 1 until height)
    )
}

fun countVisibleTrees(): Int {
    val input = File("input.txt").readLines()
    val stride = input[0].length
    val height = input.size
    return input.flatMapIndexed { y, s ->
        s.mapIndexed columnIter@ { x, tree ->
            val h = tree.digitToInt()
            dirIter@ for ((dx, dy) in getDirections(x, y, stride, height)) {
                for (px in dx) {
                    for (py in dy) {
                        if (input[py][px].digitToInt() >= h) {
                            continue@dirIter
                        }
                    }
                }
                return@columnIter y * stride + x
            }
        }
    }.distinct().size
}

fun findHighestScenicScore(): Int {
    val input = File("input.txt").readLines()
    val stride = input[0].length
    val height = input.size
    var max = 0
    input.forEachIndexed { y, s ->
        s.forEachIndexed inner@ { x, tree ->
            val h = tree.digitToInt()
            val score = getDirections(x, y, stride, height).map {  (dx, dy) ->
                var treeCount = 0
                loop@ for (px in dx) {
                    for (py in dy) {
                        val tHeight = input[py][px].digitToInt()
                        if (tHeight <= h) {
                            treeCount++
                        }
                        if (tHeight >= h) {
                            break@loop
                        }
                    }
                }
                treeCount
            }.fold(1) { acc, next -> acc * next }
            if (score > max) {
                max = score
            }
        }
    }
    return max
}

fun part1() = countVisibleTrees()
fun part2() = findHighestScenicScore()

fun main() {
    println("part 1 answer: ${part1()}")
    println("part 2 answer: ${part2()}")
}
