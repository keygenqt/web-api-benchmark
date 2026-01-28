package com.benchmark.utils

import com.benchmark.db.models.UserAuth
import io.ktor.server.sessions.SessionTransportTransformerEncrypt
import io.ktor.server.sessions.SessionsConfig
import io.ktor.server.sessions.cookie

/**
 * Configure session
 */
fun SessionsConfig.session(
    secret: String,
) {
    cookie<UserAuth>(Constants.SESSION_NAME) {
        cookie.path = "/"
        cookie.httpOnly = true
        cookie.maxAgeInSeconds = 43200 // 12 hours
        transform(
            SessionTransportTransformerEncrypt(
                secret.take(16).toByteArray(),
                secret.take(16).toByteArray()
            )
        )
    }
}
