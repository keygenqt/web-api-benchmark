package com.benchmark.db.services

import com.benchmark.db.entities.Data
import com.benchmark.db.entities.DataEntity
import com.benchmark.db.transaction
import com.benchmark.models.DataModel
import org.jetbrains.exposed.sql.Database

/**
 * Service entity data table.
 */
class DataService(private val database: Database) {
    /**
     * Find entity by nuber.
     */
    suspend fun findByNumber(id: Int): List<DataModel> = database.transaction {
        DataEntity.find { (Data.title like "%$id%") }.map { DataModel(it.title, it.createAt) }
    }
}
