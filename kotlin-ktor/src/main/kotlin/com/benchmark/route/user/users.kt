package com.benchmark.route.user

import com.benchmark.app.Exceptions
import com.benchmark.db.models.UserModel
import com.benchmark.db.services.UserService
import com.benchmark.utils.getNumberParam
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.koin.ktor.ext.inject
import io.ktor.resources.*
import io.ktor.server.resources.get as resGet

/**
 * Query params
 */
@Resource("/users")
class SearchParams(
    val fname: String? = null,
    val lname: String? = null,
)

/**
 * GET find by fname and lname
 */
fun Route.usersSearch() {
    val dataService: UserService by inject()
    resGet<SearchParams> { search ->
        // check
        if (search.fname == null && search.lname == null) {
            call.respond(emptyList<UserModel>())
            return@resGet
        }
        // act
        val response = dataService.search(search.fname, search.lname)
        // response
        call.respond(response)
    }
}

/**
 * GET find by user ID
 */
fun Route.usersFindById() {
    val dataService: UserService by inject()
    route("/users/{id}") {
        get {
            val id = call.getNumberParam()
            // act
            val response = dataService.findById(id) ?: throw Exceptions.NotFound()
            // response
            call.respond(response)
        }
    }
}
