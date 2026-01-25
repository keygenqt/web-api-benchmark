package com.benchmark.cli

import com.benchmark.app.AppConfig
import com.benchmark.route.configureServer
import com.typesafe.config.ConfigFactory
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.hocon.Hocon
import kotlinx.serialization.hocon.decodeFromConfig
import picocli.CommandLine
import picocli.CommandLine.Model.CommandSpec
import picocli.CommandLine.Spec
import java.io.File
import java.util.concurrent.Callable

/**
 * Command for start server
 */
@CommandLine.Command(
    name = "start",
    aliases = ["-s"],
    description = ["Start server API"],
)
class StartArgs : Callable<Int> {
    @Spec
    lateinit var spec: CommandSpec

    @CommandLine.Option(names = ["-h", "--help"], description = ["Show this help message and exit."])
    private var help: Boolean = false

    @CommandLine.Option(names = ["--config", "-c"], description = ["Path to the configuration file."])
    private val path: File? = null

    @OptIn(ExperimentalSerializationApi::class)
    override fun call(): Int {
        // Show help
        if (help || path == null) {
            spec.commandLine().usage(System.out)
            return 0
        }

        // Check path to configuration
        if (!path.exists() || !path.isFile) {
            throw IllegalArgumentException("File configuration not found.")
        }

        // Load configuration
        val config = ConfigFactory.parseString(path.readText())
        val appConfig = Hocon.decodeFromConfig<AppConfig>(config)

        // Start server
        embeddedServer(Netty, port = appConfig.deployment.port, host = appConfig.deployment.host) {
            configureServer(appConfig)
        }.start(wait = true)

        return 0
    }
}
