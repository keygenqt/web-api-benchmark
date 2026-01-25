package com.benchmark.db.services

import com.benchmark.db.entities.User
import com.benchmark.db.entities.UserEntity
import com.benchmark.db.entities.UserModel
import com.benchmark.db.transaction
import org.jetbrains.exposed.sql.Database

/**
 * Service entity users table
 */
class UserService(private val database: Database) {
    /**
     * Find all register user
     */
    suspend fun findAll(): List<UserModel> = database.transaction {
        UserEntity.find { (User.password.isNotNull() ) }.map {
            UserModel(
                email = it.email,
                fname = it.fname,
                lname = it.lname,
                createAt = it.createAt.toEpochMilli(),
            )
        }
    }
}
