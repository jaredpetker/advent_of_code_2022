import java.io.File

fun findMarker(input: String, uniqueChars: Int): Int {
    return (0..input.length - uniqueChars).indexOfFirst {
        input.subSequence(it until it + uniqueChars).chars().distinct().count().toInt() == uniqueChars
    } + uniqueChars
}

fun part1(input: String) = findMarker(input, 4)
fun part2(input: String) = findMarker(input, 14)

fun main() {
    val input = File("input.txt").readText(Charsets.UTF_8)
    println(part1(input))
    println(part2(input))
}
