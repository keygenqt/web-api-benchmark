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

    @CommandLine.Option(names = ["-c", "--config"], description = ["Path to the configuration file."])
    private val config: File? = null

    @OptIn(ExperimentalSerializationApi::class)
    override fun call(): Int {
        // show help
        if (help || config == null) {
            spec.commandLine().usage(System.out)
            return 0
        }
        // check path to configuration
        if (!config.exists() || !config.isFile) {
            throw IllegalArgumentException("File configuration not found.")
        }
        // load configuration
        val config = ConfigFactory.parseString(config.readText())
        val appConfig = Hocon.decodeFromConfig<AppConfig>(config)
        // start server
        embeddedServer(Netty, port = appConfig.deployment.port, host = appConfig.deployment.host) {
            configureServer(appConfig)
            if (!appConfig.server.development) {
                println("The server has started. Listening on http://${appConfig.deployment.host}:${appConfig.deployment.port}")
            }
        }.start(wait = true)
        // exit success
        return 0
    }
}
