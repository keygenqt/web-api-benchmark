package com.benchmark.utils

import kotlinx.serialization.Serializable

@Serializable
data class StatusResponse(val code: Int, val message: String)
