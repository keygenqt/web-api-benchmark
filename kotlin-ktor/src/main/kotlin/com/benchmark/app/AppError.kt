package com.benchmark.app

import io.ktor.http.HttpStatusCode
import kotlinx.serialization.Serializable

/**
 * Model
 */
@Serializable
data class ExceptionModel(
    val code: Int,
    val message: String,
)

/**
 * Errors.
 */
sealed class Exceptions(
    open val status: HttpStatusCode,
) : RuntimeException() {
    data class NotFound(
        override val status: HttpStatusCode = HttpStatusCode.NotFound
    ) : Exceptions(status)
}
