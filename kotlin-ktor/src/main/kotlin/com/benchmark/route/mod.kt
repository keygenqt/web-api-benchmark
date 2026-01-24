package com.benchmark.db

import com.benchmark.app.AppConfig
import kotlinx.coroutines.Dispatchers
import org.flywaydb.core.Flyway
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.transactions.experimental.newSuspendedTransaction

/**
 * Init database.
 */
fun configureDatabase(
    appConfig: AppConfig
): Database {
    return Database.connectWithFlyway(
        url = appConfig.postgres.url,
        user = appConfig.postgres.user,
        password = appConfig.postgres.password,
        migration = "com/benchmark/migration"
    ) ?: throw RuntimeException("Error connect to DB!")
}

/**
 * Init migration.
 */
private fun Database.Companion.connectWithFlyway(
    url: String,
    user: String,
    password: String,
    migration: String
): Database? {
    return try {
        val database = connect(
            url = url,
            user = user,
            password = password,
        )
        val flyway = Flyway.configure()
            .locations(migration)
            .dataSource(url, user, password)
            .load()
        flyway.info()
        flyway.migrate()
        database
    } catch (e: Exception) {
        null
    }
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
