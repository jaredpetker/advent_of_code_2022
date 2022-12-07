import java.io.File
import java.util.*

const val MAX_FILE_SIZE_SUM = 100000
const val AVAILABLE_SPACE = 70000000
const val DESIRED_UNUSED_SPACE = 30000000

typealias Resources = Vector<Resource>

sealed class ResourceProps {
    data class File(val name: String, val size: Int) : ResourceProps()
    data class Dir(val name: String) : ResourceProps()
}

sealed class Resource {
    companion object {}

    abstract fun print(depth: Int = 0)
    data class File(val name: String, val size: Int, val parent: Dir) : Resource() {
        override fun print(depth: Int) {
            println(" ".repeat(depth * 2) + name)
        }
    }

    data class Dir(val name: String, val parent: Dir? = null, val resources: Resources = Resources()) : Resource() {
        override fun print(depth: Int) {
            println(" ".repeat(depth * 2) + name)
            resources.forEach { it.print(depth + 1) }
        }

        fun addResource(props: ResourceProps) {
            when (props) {
                is ResourceProps.Dir -> {
                    resources.add(Dir(props.name, this))
                }

                is ResourceProps.File -> {
                    resources.add(File(props.name, props.size, this))
                }
            }
        }

        fun getDir(name: String): Dir? {
            if (name == "..") {
                return parent
            }
            return resources.find { (it is Dir) && it.name == name } as Dir?
        }

        fun shallowSize(): Int {
            return resources.filterIsInstance<File>().sumOf { it.size }
        }
    }
}

sealed class Command {
    companion object {
        val CommandRegex = Regex("[$]\\s(\\w+)\\s?(.*)?")
    }
    data class ChangeDir(val dir: String) : Command()
    class ListDir : Command() {
        private val DirRegex = Regex("^dir\\s(\\w+)$")
        private val FileRegex = Regex("^(\\d+)\\s(.*)$")
        fun process(line: String): ResourceProps {
            DirRegex.find(line)?.let {
                return ResourceProps.Dir(it.groupValues[1])
            }
            FileRegex.find(line)?.let {
                return ResourceProps.File(it.groupValues[2], it.groupValues[1].toInt())
            }
            throw Exception("Cannot process resource for line $line")
        }
    }
}

fun Command.Companion.parse(line: String): Command {
    val match = CommandRegex.find(line)!!
    return if (match.groupValues[1] == "cd") {
        Command.ChangeDir(match.groupValues[2])
    } else if (match.groupValues[1] == "ls") {
        Command.ListDir()
    } else {
        throw Exception("Command Not Found $line")
    }
}

fun getDirSizes(dir: Resource.Dir, path: String = "", dirMap: HashMap<String, Int>): Int {
    val childrenSize = dir.resources.filterIsInstance<Resource.Dir>().sumOf {
        getDirSizes(it, arrayOf(path, dir.name).joinToString("/"), dirMap)
    }
    val size = dir.shallowSize() + childrenSize
    dirMap[arrayOf(path, dir.name).joinToString("/")] = size
    return size
}

fun main() {
    val root = Resource.Dir("")
    var currDir = root

    val iter = File("input.txt").readLines().listIterator()
    loop@ while (iter.hasNext()) {
        val line = iter.next()
        when (val cmd = Command.parse(line)) {
            is Command.ChangeDir -> {
                if (cmd.dir == "/") {
                    currDir = root
                } else {
                    currDir.getDir(cmd.dir)?.let {
                        currDir = it
                    }
                }
            }

            is Command.ListDir -> {
                while (iter.hasNext()) {
                    val nextLine = iter.next()
                    if (nextLine.startsWith('$')) {
                        iter.previous()
                        continue@loop
                    }
                    val props = cmd.process(nextLine)
                    currDir.addResource(props)
                }
            }
        }
    }

    val sizesMap = HashMap<String, Int>()
    val currentUsedSpace = getDirSizes(root, dirMap = sizesMap)
    val sumOfDirsUnderLimit = sizesMap.values.filter { it <= MAX_FILE_SIZE_SUM }.sum()
    val spaceRequired = DESIRED_UNUSED_SPACE - (AVAILABLE_SPACE - currentUsedSpace)
    val spaceToFree = sizesMap.minBy { entry ->
        if (entry.value - spaceRequired >= 0) entry.value - spaceRequired else Int.MAX_VALUE
    }.value

    //part1
    println("part 1: $sumOfDirsUnderLimit")

    // part 2
    println("part 2: $spaceToFree")
}
