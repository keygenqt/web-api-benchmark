package com.benchmark.cli

import com.benchmark.app.AppInfo
import picocli.CommandLine
import picocli.CommandLine.Model.CommandSpec
import picocli.CommandLine.Spec
import java.util.concurrent.Callable

/**
 * Command interface line
 */
@CommandLine.Command(
    name = AppInfo.NAME,
    version = ["Version: ${AppInfo.VERSION}"],
    description = [AppInfo.ABOUT],
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
    @Spec
    lateinit var spec: CommandSpec

    override fun call(): Int {
        if (args.isEmpty()) {
            spec.commandLine().usage(System.out)
            return 0
        }
        return 0
    }
}
