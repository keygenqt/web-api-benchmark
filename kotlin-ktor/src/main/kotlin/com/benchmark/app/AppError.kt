package com.benchmark.app

import io.ktor.http.*
import kotlinx.serialization.Serializable

/**
 * Application custom Error
 */
@Serializable
data class AppError(
    val code: Int,
    val message: String,
)

/**
 * Errors
 */
sealed class Exceptions(
    open val status: HttpStatusCode,
) : RuntimeException() {
    data class NotFound(
        override val status: HttpStatusCode = HttpStatusCode.NotFound
    ) : Exceptions(status)

    data class Unauthorized(
        override val status: HttpStatusCode = HttpStatusCode.Unauthorized
    ) : Exceptions(status)
}
