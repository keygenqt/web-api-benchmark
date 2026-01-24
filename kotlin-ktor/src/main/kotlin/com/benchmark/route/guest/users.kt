package com.benchmark.route.guest

import com.benchmark.db.services.UserService
import com.benchmark.utils.getNumberParam
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.koin.ktor.ext.inject

fun Route.guestData() {

    val dataService: UserService by inject()

    route("/data") {
        get("/{id}") {
            // get request
            val id = call.getNumberParam()
            // act
            val response = dataService.findByNumber(id)
            // response
            call.respond(response)
        }
    }
}
