import java.io.File
import java.lang.Integer.max

data class Valve constructor(val name: String, var flowRate: Int, val adjValves: Array<String>)

class DFS constructor(
    val valves: HashMap<String, Valve>,
    val distances: HashMap<String, HashMap<String, Int>>,
    val cache: HashMap<String, Int> = hashMapOf(),
    val bitmap: HashMap<String, Int> = run {
        val map = HashMap<String, Int>()
        var i = 0
        for (entry in valves.entries) {
            map[entry.key] = i++
        }
        map
    }
) {
    fun dfs(name: String, mins: Int, visited: MutableSet<String> = LinkedHashSet()): Int {
        var max = 0
        visited.add(name)
        for ((nzValveName, dist) in distances[name]!!.entries) {
            if (!visited.contains(nzValveName)) {
                val copiedVisited = visited.toMutableSet()
                val timeAfterOpen = (mins - dist - 1)
                if (timeAfterOpen > 0) {
                    val total =
                        dfs(nzValveName, timeAfterOpen, copiedVisited) + timeAfterOpen * valves[nzValveName]!!.flowRate
                    max = max.coerceAtLeast(total)
                }
            }
        }
        return max
    }

    fun dfsBits(name: String, mins: Int, visited: Int = 0): Int {
        var maxRate = 0
        val nextVisited = visited or (1 shl bitmap[name]!!)
        for ((nzValveName, dist) in distances[name]!!.entries) {
            if ((nextVisited and (1 shl bitmap[nzValveName]!!)) == 0) {
                val timeAfterOpen = max((mins - dist - 1), 0)
                if (timeAfterOpen > 0) {
                    val key = nzValveName + timeAfterOpen + nextVisited
                    val total = cache.getOrPut(key) {
                        dfsBits(
                            nzValveName, timeAfterOpen, nextVisited
                        )
                    } + timeAfterOpen * valves[nzValveName]!!.flowRate
                    maxRate = maxRate.coerceAtLeast(total)
                }
            }
        }
        return maxRate
    }

    fun dfsWithElephant(names: Pair<String, String>, mins: Pair<Int, Int>, visited: Int = 0): Int {
        var maxRate = 0
        val nextVisited = visited or (1 shl bitmap[names.first]!!) or (1 shl bitmap[names.second]!!)
        for ((nzValveName, dist) in distances[names.first]!!.entries) {
            for ((nzValveNameEl, distEl) in distances[names.second]!!.entries) {
                if (nzValveName != nzValveNameEl) {
                    if ((nextVisited and (1 shl bitmap[nzValveName]!!)) == 0 && (nextVisited and (1 shl bitmap[nzValveNameEl]!!)) == 0) {
                        val timeAfterOpen = max((mins.first - dist - 1), 0)
                        val timeAfterOpenEl = max((mins.second - distEl - 1), 0)
                        if (timeAfterOpen + timeAfterOpenEl > 0) {
                            val key = nzValveName + nzValveNameEl + timeAfterOpen + timeAfterOpenEl + nextVisited
                            val t = cache.getOrPut(key) {
                                dfsWithElephant(
                                    Pair(nzValveName, nzValveNameEl),
                                    Pair(timeAfterOpen, timeAfterOpenEl),
                                    nextVisited
                                )
                            }
                            val total =
                                t + timeAfterOpen * valves[nzValveName]!!.flowRate + timeAfterOpenEl * valves[nzValveNameEl]!!.flowRate
                            maxRate = maxRate.coerceAtLeast(total)
                        }
                    }
                }
            }
        }
        return maxRate
    }
}


fun main() {
    val valves = hashMapOf<String, Valve>()
    val lines = File("input.txt").readLines()
    lines.asSequence().forEach { line ->
        val matchSeq = Regex("[A-Z]{2}|\\d+").findAll(line).toList()
        val name = matchSeq[0].value
        val flow = matchSeq[1].value.toInt()
        val adjValves = matchSeq.slice(2 until matchSeq.size).map { it.value }.toTypedArray()
        valves[name] = Valve(name, flow, adjValves)
    }

    // build a structure for distances from every non-0 flow valve to every other non-zero flow valve
    val distances: HashMap<String, HashMap<String, Int>> = hashMapOf()
    for (valve in valves.values) {
        if (valve.name != "AA" && valve.flowRate == 0) {
            continue
        }

        val queue = ArrayDeque<Pair<String, Int>>()
        val visited = HashSet<String>()
        queue.add(Pair(valve.name, 0))
        visited.add(valve.name)
        while (queue.isNotEmpty()) {
            val (name, dist) = queue.removeFirst()
            val curr = valves[name]!!
            for (adjValve in curr.adjValves) {
                if (!visited.contains(adjValve)) {
                    visited.add(adjValve)
                    queue.add(Pair(adjValve, dist + 1))
                    if (valves[adjValve]!!.flowRate > 0) {
                        distances.putIfAbsent(valve.name, hashMapOf())
                        distances[valve.name]!![adjValve] = dist + 1
                    }
                }
            }
        }
    }

    println(DFS(valves, distances).dfs("AA", 30))
    println(DFS(valves, distances).dfsWithElephant(Pair("AA", "AA"), Pair(26, 26)))
}
