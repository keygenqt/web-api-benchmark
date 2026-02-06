package com.benchmark.app

import io.ktor.http.*
import kotlinx.serialization.Serializable

/**
 * Errors
 */
sealed class Exceptions(
    open val status: HttpStatusCode,
) : RuntimeException() {
    data class NotFound(
        override val status: HttpStatusCode = HttpStatusCode.NotFound
    ) : Exceptions(status)

    data class Forbidden(
        override val status: HttpStatusCode = HttpStatusCode.Forbidden
    ) : Exceptions(status)
}
