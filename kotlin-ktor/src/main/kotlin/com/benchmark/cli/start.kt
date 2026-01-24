package com.benchmark.app

import picocli.CommandLine
import picocli.CommandLine.Model.CommandSpec
import picocli.CommandLine.Spec
import java.io.File
import java.util.concurrent.Callable

@CommandLine.Command(
    name = "start",
    aliases = ["-s"],
    description = ["Start server API"],
)
class StartArgs : Callable<Int> {
    @Spec lateinit var spec: CommandSpec

    @CommandLine.Option(names = ["-h", "--help"], description = ["Show this help message and exit."])
    private var help: Boolean = false

    @CommandLine.Option(names =["--config", "-c"], description = ["Path to the configuration file."])
    private val path: File? = null

    override fun call(): Int {
        if (help || path == null) {
            spec.commandLine().usage(System.out)
            return 0
        }
        val configuration = path.readText()

        // @todo run server

        return 0
    }
}
