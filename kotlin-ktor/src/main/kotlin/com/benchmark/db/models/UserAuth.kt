package com.benchmark.db.models

import kotlinx.serialization.Serializable

/**
 * User data session
 */
@Serializable
data class UserAuth(
    val id: Int,
    val email: String
)
