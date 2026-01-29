package com.benchmark.utils

import kotlinx.serialization.Serializable

/**
 * Common response
 */
@Serializable
data class StatusResponse(val code: Int, val message: String)
