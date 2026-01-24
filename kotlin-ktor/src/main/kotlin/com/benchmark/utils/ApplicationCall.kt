package com.benchmark

import com.benchmark.app.Exceptions
import io.ktor.server.application.ApplicationCall

/**
 * Get value from params with validate
 */
fun ApplicationCall.getNumberParam(key: String = "id"): Int = parameters[key]
    ?.toIntOrNull() ?: throw Exceptions.NotFound()
