package com.benchmark.route.guest

import com.benchmark.db.services.UserService
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.koin.ktor.ext.inject

/**
 * Users routing for guests.
 */
fun Route.guestUsers() {
    val dataService: UserService by inject()
    route("/users") {
        get {
            // act
            val response = dataService.findAll()
            // response
            call.respond(response)
        }
    }
}
