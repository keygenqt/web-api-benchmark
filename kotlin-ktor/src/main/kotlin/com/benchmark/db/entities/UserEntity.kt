package com.benchmark.db.entities

import com.benchmark.db.models.UserModel
import org.jetbrains.exposed.v1.core.dao.id.EntityID
import org.jetbrains.exposed.v1.core.dao.id.IntIdTable
import org.jetbrains.exposed.v1.dao.IntEntity
import org.jetbrains.exposed.v1.dao.IntEntityClass
import org.jetbrains.exposed.v1.javatime.CurrentTimestamp
import org.jetbrains.exposed.v1.javatime.timestamp

/**
 * Table users
 */
object User : IntIdTable("users_ktor") {
    val email = varchar("email", length = 255).uniqueIndex()
    val fname = varchar("fname", length = 255)
    val lname = varchar("lname", length = 255)
    val pwdHash = varchar("pwd_hash", length = 255)
    val createAt = timestamp("create_at").defaultExpression(CurrentTimestamp)
}

/**
 * Entity
 */
class UserEntity(id: EntityID<Int>) : IntEntity(id) {
    companion object : IntEntityClass<UserEntity>(User)

    var email by User.email
    var fname by User.fname
    var lname by User.lname
    var pwdHash by User.pwdHash
    var createAt by User.createAt
}

fun UserEntity.toModel() = UserModel(
    id = id.value,
    email = email,
    fname = fname,
    lname = lname,
    createAt = createAt.toEpochMilli(),
)
