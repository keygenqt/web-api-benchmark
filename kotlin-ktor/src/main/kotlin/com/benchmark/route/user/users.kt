package com.benchmark.route.user

import com.benchmark.db.services.UserService
import com.benchmark.utils.getNumberParam
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.koin.ktor.ext.inject

/**
 * GET find by fname and lname
 */
fun Route.usersSearch() {
    val dataService: UserService by inject()
    route("/users") {
        get {
//            // act
//            val response = dataService.findAll()
//            // response
//            call.respond(response)
        }
    }
}

/**
 * GET find by user ID
 */
fun Route.usersFindById() {
    val dataService: UserService by inject()
    route("/users/{id}") {
        get {
//            val id = call.getNumberParam()
//            // act
//            val response = dataService.findById(id)
//            // response
//            call.respond(response)
        }
    }
}
