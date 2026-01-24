package com.benchmark.utils

import com.benchmark.app.Exceptions
import io.ktor.server.application.*

/**
 * Get value from params with validate
 */
fun ApplicationCall.getNumberParam(key: String = "id"): Int = parameters[key]
    ?.toIntOrNull() ?: throw Exceptions.NotFound()
