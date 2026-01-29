package com.benchmark.route

import ch.qos.logback.classic.Logger
import com.benchmark.app.AppConfig
import com.benchmark.app.AppError
import com.benchmark.app.Exceptions
import com.benchmark.db.configureDatabase
import com.benchmark.db.services.UserService
import com.benchmark.route.guest.authBasic
import com.benchmark.route.guest.authLogout
import com.benchmark.route.user.usersFindById
import com.benchmark.route.user.usersSearch
import com.benchmark.utils.Constants
import com.benchmark.utils.auth
import com.benchmark.utils.session
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.http.content.*
import io.ktor.server.plugins.calllogging.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.resources.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.sessions.*
import kotlinx.serialization.json.Json
import org.koin.dsl.module
import org.koin.ktor.plugin.Koin
import org.slf4j.Logger.ROOT_LOGGER_NAME
import org.slf4j.LoggerFactory
import ch.qos.logback.classic.Level as LogbackLevel
import org.koin.core.logger.Level as KoinLevel
import org.slf4j.event.Level as Slf4jLevel

/**
 * Init Ktor server
 */
fun Application.configureServer(
    config: AppConfig
) {
    // init logger
    configureLogging(config.server.development)
    // init database
    val database = configureDatabase(config)
    // init DI Koin
    install(Koin) {
        printLogger(if (config.server.development) KoinLevel.DEBUG else KoinLevel.NONE)
        modules(
            module {
                single { config }
                single { UserService(database) }
            }
        )
    }
    // init response
    install(ContentNegotiation) {
        json(
            Json {
                isLenient = true
                ignoreUnknownKeys = true
                coerceInputValues = true
            }
        )
    }
    // init catch errors
    install(StatusPages) {
        configureHandleError()
    }
    // init session
    install(Sessions) {
        session(config.server.secret)
    }
    // init auth
    install(Authentication) {
        auth()
    }
    // init Type-safe routing
    install(Resources)
    // init Routing
    routing {
        staticResources("/", "assets") {
            enableAutoHeadResponse()
        }
        route("/api") {
            authBasic()
            authLogout()
            authenticate(Constants.SESSION_NAME) {
                usersSearch()
                usersFindById()
            }
        }
    }
}

/**
 * Configure tracing
 */
private fun Application.configureLogging(
    isDevelopment: Boolean
) {
    // logger db
    (LoggerFactory.getLogger(ROOT_LOGGER_NAME) as Logger).apply {
        level = if (isDevelopment) LogbackLevel.DEBUG else LogbackLevel.OFF
    }
    // configuration
    install(CallLogging) {
        level = if (isDevelopment) Slf4jLevel.DEBUG else Slf4jLevel.ERROR
    }
}

/**
 * Catch errors
 */
private fun StatusPagesConfig.configureHandleError() {
    status(HttpStatusCode.Unauthorized, HttpStatusCode.NotFound) { call, cause ->
        call.respond(
            status = cause,
            message = AppError(
                code = cause.value,
                message = cause.description
            )
        )
    }
    exception<Throwable> { call, cause ->
        if (cause is Exceptions) {
            call.respond(
                status = cause.status,
                message = AppError(
                    code = cause.status.value,
                    message = cause.status.description
                )
            )
        } else {
            call.respond(
                status = HttpStatusCode.InternalServerError,
                message = AppError(
                    code = HttpStatusCode.InternalServerError.value,
                    message = cause.message ?: HttpStatusCode.InternalServerError.description
                )
            )
        }
    }
}
