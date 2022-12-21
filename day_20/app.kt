import java.io.File
import java.util.*
import kotlin.math.absoluteValue
import kotlin.math.sign

fun grovePos(n: Int, index: Int, size: Int) = (n + index) % size

data class Node(val v: Long, val p: Int)

fun main() {
    for ((part, times) in arrayOf(1, 10).withIndex()) {
        val mult = if (part == 0) { 1 } else { 811589153 }
        val numbers = File("input.txt").readLines()
            .mapIndexedTo(LinkedList()) { i, v -> Node(v.toLong() * mult, i) }
        val re: LinkedList<Node> = numbers.clone() as LinkedList<Node>
        for (i in 0 until times) {
            for (node in numbers) {
                val (num, p) = node
                if (num != 0L) {
                    val index = re.indexOfFirst { it.p == p }
                    var n = num
                    if (n.absoluteValue >= re.size - 1) {
                        n %= (re.size - 1)
                    }
                    var pos: Long = index + n
                    if (pos >= re.size) {
                        pos -= re.size
                    } else if (pos < 0) {
                        pos += re.size
                    }
                    val offset = if (num.sign == 1) {
                        1
                    } else {
                        0
                    }

                    if (pos > index) {
                        re.add(pos.toInt() + offset, node)
                        re.removeAt(index)
                    } else if (pos < index) {
                        re.removeAt(index)
                        re.add(pos.toInt() + offset, node)
                    }
                }
            }
        }
        val zi = re.indexOfFirst { it.v == 0L }
        println(
            "part ${part + 1} answer: ${
                arrayOf(1000, 2000, 3000).sumOf {
                    re.elementAt(
                        grovePos(
                            it,
                            zi,
                            re.size
                        )
                    ).v
                }
            }"
        )
    }

