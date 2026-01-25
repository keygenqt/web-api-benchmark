package com.benchmark.db

import com.benchmark.app.AppConfig
import kotlinx.coroutines.Dispatchers
import org.flywaydb.core.Flyway
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.Database.Companion.connect
import org.jetbrains.exposed.sql.transactions.experimental.newSuspendedTransaction
import java.nio.file.Paths

/**
 * Init database
 */
fun configureDatabase(
    appConfig: AppConfig
): Database {
    val database = connect(
        url = appConfig.postgres.url,
        user = appConfig.postgres.user,
        password = appConfig.postgres.password,
    )
    val cwd = Paths.get("").toAbsolutePath()
    val locations = mutableListOf("filesystem:${cwd}/migrations").apply {
        if (appConfig.server.development) {
            add("filesystem:${cwd}/../../migrations")
        }
    }
    val flyway = Flyway.configure()
        .sqlMigrationPrefix("")
        .sqlMigrationSeparator("_")
        .locations(*locations.toTypedArray())
        .dataSource(appConfig.postgres.url, appConfig.postgres.user, appConfig.postgres.password)
        .load()
    flyway.info()
    flyway.migrate()
    return database
}

/**
 * Extension transaction.
 */
suspend fun <T> Database.transaction(block: suspend () -> T): T {
    return newSuspendedTransaction(
        context = Dispatchers.IO,
        db = this
    ) { block() }
}
