package com.benchmark.cli

import ch.qos.logback.classic.Level
import ch.qos.logback.classic.Logger
import com.benchmark.app.AppConfig
import com.benchmark.db.configureDatabase
import com.benchmark.db.models.UserModel
import com.benchmark.db.services.UserService
import com.typesafe.config.ConfigFactory
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.hocon.Hocon
import kotlinx.serialization.hocon.decodeFromConfig
import kotlinx.serialization.json.Json
import org.slf4j.Logger.ROOT_LOGGER_NAME
import org.slf4j.LoggerFactory
import picocli.CommandLine
import picocli.CommandLine.Model.CommandSpec
import picocli.CommandLine.Spec
import java.io.File
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
    @Spec
    lateinit var spec: CommandSpec

    @CommandLine.Option(names = ["-h", "--help"], description = ["Show this help message and exit."])
    private var help: Boolean = false

    @CommandLine.Option(names = ["-c", "--config"], description = ["Path to the configuration file."])
    private val config: File? = null

    @CommandLine.Option(names = ["-u", "--users"], description = ["Import users from JSON."])
    private val users: File? = null

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
        // run users import
        if (users != null) {
            return importUsers(appConfig, users)
        }
        // exit success
        return 0
    }

    fun importUsers(config: AppConfig, pathJson: File): Int {
        if (!pathJson.exists() || !pathJson.isFile) {
            throw IllegalArgumentException("File JSON for import users not found.")
        }
        // init db logger level
        (LoggerFactory.getLogger(ROOT_LOGGER_NAME) as Logger).apply {
            level = Level.OFF
        }
        // init database
        val database = configureDatabase(config)
        // init user service db
        val userService = UserService(database)
        // read json, parse
        val usersJson: List<UserModel> = Json.decodeFromString(pathJson.readText())
        // loop users
        usersJson.forEach { model ->
            val find = runBlocking { userService.findByEmail(email = model.email) }
            if (find != null) {
                println("""✗ user ${find.email}: already exists""")
                return@forEach
            }
            val user = model.updateHash()
            try {
                runBlocking {
                    userService.insert(
                        email = user.email,
                        fname = user.fname,
                        lname = user.lname,
                        pwdHash = user.pwdHash!!,
                    )
                    println("""✓ user ${user.email}""")
                }
            } catch (e: Exception) {
                println(
                    """✗ user ${user.email}: ${
                        e.message?.split("\n")?.lastOrNull()?.trim()?.split("  ")?.firstOrNull()
                    }"""
                )
            }
        }
        return 0
    }
}
