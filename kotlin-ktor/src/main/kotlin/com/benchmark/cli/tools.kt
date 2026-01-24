package com.benchmark.cli

import picocli.CommandLine
import picocli.CommandLine.Model.CommandSpec
import picocli.CommandLine.Spec
import java.util.concurrent.Callable

/**
 * Command for work on server
 */
@CommandLine.Command(
    name = "tools",
    aliases = ["-t"],
    description = ["Tools for server"]
)
class ToolsArgs : Callable<Int> {
    @Spec lateinit var spec: CommandSpec

    @CommandLine.Option(names = ["-h", "--help"], description = ["Show this help message and exit."])
    private var help: Boolean = false

    @CommandLine.Option(names =["--stats", "-s"], description = ["Display data statistics."])
    private var stats: Boolean? = null

    override fun call(): Int {
        if (help || stats == null) {
            spec.commandLine().usage(System.out)
            return 0
        }
        if (stats == true) {
            println("Run stats")
        }
        return 0
    }
}
