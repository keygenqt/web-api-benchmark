package com.benchmark.db.entities

import kotlinx.serialization.Serializable
import org.jetbrains.exposed.dao.IntEntity
import org.jetbrains.exposed.dao.IntEntityClass
import org.jetbrains.exposed.dao.id.EntityID
import org.jetbrains.exposed.dao.id.IntIdTable
import org.jetbrains.exposed.sql.javatime.CurrentTimestamp
import org.jetbrains.exposed.sql.javatime.timestamp

/**
 * Table users.
 */
object User : IntIdTable("users_ktor") {
    val email = varchar("email", length = 255).uniqueIndex()
    val fname = varchar("fname", length = 255)
    val lname = varchar("lname", length = 255)
    val passwd = varchar("passwd", length = 255)
    val createAt = timestamp("created_at").defaultExpression(CurrentTimestamp)
}

/**
 * Entity.
 */
class UserEntity(id: EntityID<Int>) : IntEntity(id) {
    companion object : IntEntityClass<UserEntity>(User)

    var email by User.email
    var fname by User.fname
    var lname by User.lname
    var passwd by User.passwd
    var createAt by User.createAt
}

/**
 * Model user entry [UserEntity].
 */
@Serializable
data class UserModel(
    val email: String,
    val fname: String,
    val lname: String,
    val createAt: Long,
)
