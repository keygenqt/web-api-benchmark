package com.benchmark.app

import kotlinx.serialization.Serializable

/**
 * Application common response
 */
@Serializable
data class StatusResponse(val code: Int, val message: String)
