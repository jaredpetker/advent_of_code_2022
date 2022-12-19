import java.io.File
import java.util.*
import kotlin.collections.ArrayList
import kotlin.math.pow

data class Point3D(val x: Int, val y: Int, val z: Int) {
    fun distance(point: Point3D): Double {
        return ((x - point.x).toDouble().pow(2) + (y - point.y).toDouble().pow(2) + (z - point.z).toDouble()
            .pow(2)).pow(1 / 2.toDouble())
    }
}

val dirs = arrayOf(
    Point3D(-1, 0, 0),
    Point3D(0, -1, 0),
    Point3D(0, 0, -1),
    Point3D(1, 0, 0),
    Point3D(0, 1, 0),
    Point3D(0, 0, 1),
)

// Start from some point (that is air) outside the lava but within a bounding cube
// Any time a point (cube) that is lava is encountered, increment that points "count"
// A cube can only be entered once from any particular point of air that is outside of it as
// long as we also keep track of which points (cubes) are air.
fun dfs(point: Point3D, bounds: Pair<Point3D, Point3D>, pointsMap: HashMap<Point3D, Int>) {
    // for the pointsMap
    // -1 means air
    // 0 or above is lava
    val queue = ArrayList<Point3D>()
    queue.add(point)
    while (queue.isNotEmpty()) {
        val p = queue.removeLast()
        val (min, max) = bounds
        if (pointsMap.contains(p)) {
            val count = pointsMap[p]
            if (count!! > -1) {
                pointsMap[p] = count + 1
            }
            continue
        } else if (p.x < min.x || p.y < min.y || p.z < min.z || p.x > max.x || p.y > max.y || p.z > max.z) {
            // out of our bounding cube
            continue
        }
        pointsMap[p] = -1
        for (dir in dirs) {
            queue.add(Point3D(p.x + dir.x, p.y + dir.y, p.z + dir.z))
        }
    }

}

fun main() {
    val points = File("input.txt").readLines()
        .map { it.split(",").let { s -> Point3D(s[0].toInt(), s[1].toInt(), s[2].toInt()) } }

    val pointsMap = HashMap<Point3D, Int>()

    // part 1
    var sides = points.size * 6
    for (i in points.indices) {
        pointsMap[points[i]] = 0
        for (j in (i + 1) until points.size) {
            val distance = points[i].distance(points[j])
            if (distance == 1.toDouble()) {
                sides -= 2
            }
        }
    }
    println("part 1 answer: $sides")

    // part 2
    val sortedX = points.sortedBy { it.x }
    val minX = (sortedX.first().x - 1)
    val maxX = (sortedX.last().x + 1)

    val sortedY = points.sortedBy { it.y }
    val minY = (sortedY.first().y - 1)
    val maxY = (sortedY.last().y + 1)

    val sortedZ = points.sortedBy { it.z }
    val minZ = (sortedZ.first().z - 1)
    val maxZ = (sortedZ.last().z + 1)

    dfs(Point3D(minX, minY, minZ), Pair(Point3D(minX, minY, minZ), Point3D(maxX, maxY, maxZ)), pointsMap)
    println("part 2 answer: ${pointsMap.values.filter { it > -1 }.sum()}")
}
