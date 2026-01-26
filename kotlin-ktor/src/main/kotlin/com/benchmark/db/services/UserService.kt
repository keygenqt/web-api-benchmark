package com.benchmark.db.services

import com.benchmark.db.entities.User
import com.benchmark.db.entities.UserEntity
import com.benchmark.db.models.UserModel
import com.benchmark.db.transaction
import org.jetbrains.exposed.v1.core.isNotNull
import org.jetbrains.exposed.v1.jdbc.Database

/**
 * Service entity users table
 */
class UserService(private val database: Database) {
    /**
     * Create entity.
     */
    suspend fun insert(
        email: String,
        fname: String,
        lname: String,
        pwdHash: String,
    ) = database.transaction {
        UserEntity.new {
            this.email = email
            this.fname = fname
            this.lname = lname
            this.pwdHash = pwdHash
        }
    }

    /**
     * Find all register user
     */
    suspend fun findAll(): List<UserModel> = database.transaction {
        UserEntity.find { (User.pwdHash.isNotNull()) }.map {
            UserModel(
                email = it.email,
                fname = it.fname,
                lname = it.lname,
                createAt = it.createAt.toEpochMilli(),
            )
        }
    }
}
