package com.benchmark.db

import com.benchmark.app.AppConfig
import org.flywaydb.core.Flyway
import org.jetbrains.exposed.v1.jdbc.Database
import org.jetbrains.exposed.v1.jdbc.Database.Companion.connect
import org.jetbrains.exposed.v1.jdbc.transactions.suspendTransaction
import java.nio.file.Paths

/**
 * Init database
 */
fun configureDatabase(
    config: AppConfig
): Database {
    // connect to db
    val database = connect(
        url = config.postgres.url,
        user = config.postgres.user,
        password = config.postgres.password,
    )
    // location migrations
    val cwd = Paths.get("").toAbsolutePath()
    val locations = mutableListOf("filesystem:${cwd}/migrations").apply {
        if (config.server.development) {
            add("filesystem:${cwd}/../../migrations")
        }
    }
    // migration
    val flyway = Flyway.configure()
        .baselineOnMigrate(true)
        .sqlMigrationPrefix("")
        .sqlMigrationSeparator("_")
        .locations(*locations.toTypedArray())
        .dataSource(config.postgres.url, config.postgres.user, config.postgres.password)
        .load()
    flyway.info()
    flyway.migrate()
    // result
    return database
}

/**
 * Extension transaction
 */
suspend fun <T> Database.transaction(block: suspend () -> T): T {
    return suspendTransaction(this) { block() }
}
