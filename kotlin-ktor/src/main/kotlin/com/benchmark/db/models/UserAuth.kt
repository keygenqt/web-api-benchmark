package com.benchmark.db.models

import kotlinx.serialization.Serializable

@Serializable
data class UserAuth(
    val id: Int,
    val email: String
)
