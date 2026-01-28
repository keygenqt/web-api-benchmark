package com.benchmark.utils

import com.benchmark.app.Exceptions
import com.benchmark.db.models.UserAuth
import io.ktor.server.auth.*

fun AuthenticationConfig.auth() {
    // @todo
    session<UserAuth>(Constants.SESSION_NAME) {
        validate { credentials ->
            throw Exceptions.Unauthorized()
        }
        challenge {
            throw Exceptions.Unauthorized()
        }
    }
}
