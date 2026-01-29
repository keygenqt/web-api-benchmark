package com.benchmark.db.services

import com.benchmark.db.entities.User
import com.benchmark.db.entities.UserEntity
import com.benchmark.db.entities.toModel
import com.benchmark.db.models.UserModel
import com.benchmark.db.transaction
import com.benchmark.utils.ilike
import org.jetbrains.exposed.v1.core.eq
import org.jetbrains.exposed.v1.jdbc.Database
import org.jetbrains.exposed.v1.jdbc.orWhere
import org.jetbrains.exposed.v1.jdbc.selectAll

/**
 * Service entity users table
 */
class UserService(private val database: Database) {
    /**
     * Search user by fname and lname
     */
    suspend fun search(
        fname: String?,
        lname: String?,
    ): List<UserModel> = database.transaction {
        User
            .selectAll()
            .apply {
                if (fname != null) {
                    orWhere { (User.fname ilike "%${fname}%") }
                }
                if (fname != null) {
                    orWhere { (User.lname ilike "%${lname}%") }
                }
            }
            .map { UserEntity.wrapRow(it) }
            .map { it.toModel() }
    }

    /**
     * Find entity by id
     */
    suspend fun findById(id: Int): UserModel? = database.transaction {
        UserEntity.findById(id)?.toModel()
    }

    /**
     * Find entity by email
     */
    suspend fun findByEmail(email: String): UserModel? = database.transaction {
        UserEntity.find { (User.email eq email) }.firstOrNull()?.toModel(true)
    }

    /**
     * Create new entity
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
}
