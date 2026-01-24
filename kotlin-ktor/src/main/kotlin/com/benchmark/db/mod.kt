package com.benchmark.cli

import picocli.CommandLine
import picocli.CommandLine.Model.CommandSpec
import picocli.CommandLine.Spec
import java.util.concurrent.Callable

/**
 * Command interface line.
 * @see <a href="https://picocli.info/">Picocli</a>
 */
@CommandLine.Command(
    name = "kotlin-ktor",
    version = ["Version: 0.0.1"],
    description = ["Kotlin Ktor - Ktor REST API Demo Application."],
    mixinStandardHelpOptions = true,
    sortOptions = false,
    subcommands = [
        StartArgs::class,
        ToolsArgs::class,
    ]
)
class AppCLI(
    private val args: Array<String>
) : Callable<Int> {
    @Spec lateinit var spec: CommandSpec

    override fun call(): Int {
        if (args.isEmpty()) {
            spec.commandLine().usage(System.out)
            return 0
        }
        return 0
    }
}
