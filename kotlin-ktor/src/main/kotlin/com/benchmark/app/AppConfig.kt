package com.benchmark.app

import kotlinx.serialization.Serializable

/**
 * Common configuration
 */
@Serializable
data class AppConfig(
    val server: ServerConfig,
    val deployment: DeploymentConfig,
    val postgres: PostgresConfig
)

/**
 * Developer params
 */
@Serializable
data class ServerConfig(
    val development: Boolean,
    val secret: String,
)

/**
 * Server run params
 */
@Serializable
data class DeploymentConfig(
    val host: String,
    val port: Int,
)

/**
 * Connect postgres params
 */
@Serializable
data class PostgresConfig(
    val url: String,
    val user: String,
    val password: String
)
