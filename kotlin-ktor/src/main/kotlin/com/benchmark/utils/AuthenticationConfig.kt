package com.benchmark.utils

import com.benchmark.app.Exceptions
import com.benchmark.db.models.UserAuth
import com.benchmark.db.services.UserService
import io.ktor.server.auth.*
import org.koin.java.KoinJavaComponent.inject

/**
 * Check session data
 */
fun AuthenticationConfig.auth() {
    val userService: UserService by inject(UserService::class.java)
    session<UserAuth>(Constants.SESSION_NAME) {
        validate { session ->
            if(userService.findById(session.id) != null) {
                session
            } else {
                null
            }
        }
        challenge {
            throw Exceptions.Forbidden()
        }
    }
}
