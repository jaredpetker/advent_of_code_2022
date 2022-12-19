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
fun findSurfaceArea(point: Point3D, bounds: Pair<Point3D, Point3D>, pointsMap: HashMap<Point3D, Int>): Int {
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
    return pointsMap.values.filter { it > -1 }.sum()
}

fun main() {
    val points = File("input.txt").readLines()
        .map { it.split(",").let { s -> Point3D(s[0].toInt(), s[1].toInt(), s[2].toInt()) } }
    val pointsMap = HashMap<Point3D, Int>()

    // part 1
    var sides = points.size * 6
    var minX = Int.MAX_VALUE
    var maxX = Int.MIN_VALUE
    var minY = Int.MAX_VALUE
    var maxY = Int.MIN_VALUE
    var minZ = Int.MAX_VALUE
    var maxZ = Int.MIN_VALUE

    for (i in points.indices) {
        pointsMap[points[i]] = 0

        val p = points[i]
        minX = p.x.coerceAtMost(minX)
        maxX = p.x.coerceAtLeast(maxX)
        minY = p.y.coerceAtMost(minY)
        maxY = p.y.coerceAtLeast(maxY)
        minZ = p.z.coerceAtMost(minZ)
        maxZ = p.z.coerceAtLeast(maxZ)

        for (j in (i + 1) until points.size) {
            val distance = points[i].distance(points[j])
            if (distance == 1.toDouble()) {
                sides -= 2
            }
        }
    }
    println("part 1 answer: $sides")

    // part 2
    val surfaceArea = findSurfaceArea(
        Point3D(minX, minY, minZ),
        Pair(Point3D(minX, minY, minZ), Point3D(maxX, maxY, maxZ)),
        pointsMap
    )
    println("part 2 answer: $surfaceArea")
}
