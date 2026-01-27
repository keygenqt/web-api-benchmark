package com.benchmark.db.models

import com.benchmark.db.entities.UserEntity
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder

/**
 * Model user entry [UserEntity]
 */
@Serializable
data class UserModel(
    val id: Int? = null,
    val email: String,
    val fname: String,
    val lname: String,
    val pwdHash: String? = null,
    val password: String? = null,
    @SerialName("create_at")
    val createAt: Long = System.currentTimeMillis(),
) {
    fun passwordValidate(password: String): Boolean {
        val encoder: Argon2PasswordEncoder = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
        return encoder.matches(password, this.password)
    }

    fun updateHash(): UserModel {
        if (password != null) {
            val encoder: Argon2PasswordEncoder = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
            val hash = encoder.encode(password)
            return this.copy(pwdHash = hash)
        }
        throw RuntimeException("Password expected")
    }
}
