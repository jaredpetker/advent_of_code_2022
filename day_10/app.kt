import java.io.File

typealias Op = (x: Int) -> Int

fun cycleOps(lines: List<String>): Sequence<Op> = sequence {
   lines.forEach { line ->
       when (line.substring(0..3)) {
           "noop" -> yield { it }
           "addx" -> {
               yield { it }
               yield {
                   val s = it + (line.substring(4).trim().toIntOrNull() ?: 0)
                   s
               }
           }
           else -> throw Exception("command not found for $line")
       }
   }
}

fun main() {
    // part 1
    val lines = File("input.txt").readLines()
    var x = 1
    val history = cycleOps(lines).mapTo(ArrayList()) { op ->
        val startValue = x
        x = op(x)
        startValue
    }
    val sum = (20..history.size step 40).sumOf {
        it * history[it - 1]
    }
    println(sum)

    // part 2
    val ops = cycleOps(lines).iterator()
    val crt = (0 until 240).foldIndexed(Pair(1, StringBuilder())) { i, acc, _ ->
        val (x, crt) = acc
        val localCycle = i % 40
        val pixel = if (x - 1 <= localCycle && localCycle <= x + 1) "#" else "."
        val op = ops.next()
        Pair(op(x), crt.append(if (localCycle == 39) "$pixel\n" else pixel))
    }.second
    println(crt)
}
