package com.benchmark.route.guest

import io.ktor.server.html.*
import io.ktor.server.routing.*
import kotlinx.html.*

/**
 * Main page HTML
 */
fun Route.guestHome() {
    get("/") {
        call.respondHtml {
            style = "width: 100%; height: 100%; background: black;"
            body {
                style = "width: 100%; height: 100%; margin: 0;"
                table {
                    style = "width: 100%; height: 100%;"
                    tr {
                        td {
                            h1 {
                                style = "color: white; text-align: center;"
                                +"Ktor - REST API"
                            }
                        }
                    }
                }
            }
        }
    }
}
