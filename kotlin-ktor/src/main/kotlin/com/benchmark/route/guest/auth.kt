package com.benchmark.route.guest

import com.benchmark.app.Exceptions
import com.benchmark.db.models.UserAuth
import com.benchmark.db.services.UserService
import com.benchmark.utils.StatusResponse
import io.ktor.http.*
import io.ktor.server.auth.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.sessions.*
import org.koin.ktor.ext.inject

/**
 * POST basic auth cookie
 */
fun Route.authBasic() {
    val dataService: UserService by inject()
    route("/auth") {
        post {
            val credentials = call.request.basicAuthenticationCredentials() ?: throw Exceptions.Unauthorized()
            // find user
            val result = dataService.findByEmail(credentials.name) ?: throw Exceptions.NotFound()
            // argon2 validate pass
            if (!result.passwordValidate(credentials.password)) {
                throw Exceptions.Unauthorized()
            }
            // get auth model
            val userAuth = UserAuth(
                id = result.id!!,
                email = result.email,
            )
            // clear session if exist
            call.sessions.clear<UserAuth>()
            // set new session
            call.sessions.set(userAuth)
            // result
            call.respond(
                StatusResponse(
                    code = HttpStatusCode.OK.value,
                    message = HttpStatusCode.OK.description
                )
            )
        }
    }
}

/**
 * DELETE logout cookie
 */
fun Route.authLogout() {
    route("/logout") {
        delete {
            // clear session if exist
            call.sessions.clear<UserAuth>()
            // result
            call.respond(
                StatusResponse(
                    code = HttpStatusCode.OK.value,
                    message = HttpStatusCode.OK.description
                )
            )
        }
    }
}
