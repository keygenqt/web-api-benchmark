package com.benchmark.route.guest

import io.ktor.server.html.*
import io.ktor.server.routing.*
import kotlinx.html.*

/**
 * GET home page
 */
fun Route.home() {
    get("/") {
        call.respondHtml {
            head {
                title { +"Welcome!" }
                style {
                    unsafe {
                        +"""
                            html, body {
                                font-family: Tahoma, Verdana, Arial, sans-serif;
                                background: #1C1C1C;
                                color: white;
                                height: 100%;
                                width: 100%;
                                padding: 0;
                                margin: 0;
                            }
                            .Root-Table {
                                padding: 0;
                                height: 100%;
                                width: 100%;
                                max-width: 700px;
                                margin: 0 auto;
                            }
                            .Root-Table .Root-Td {
                                padding: 30px;
                            }
                            .Box {
                                background-color: #00000096;
                                border: 1px solid #7F52FF;
                                border-radius: 20px;
                                padding: 40px;
                                text-align: center;
                            }
                            .Box h1 {
                                margin: 0;
                            }
                            .Box .Icon {
                                width: 150px;
                                height: 150px;
                                display: flex;
                                justify-content: center;
                                justify-items: center;
                                margin: 0 auto;
                            }
                            .Box .Icon img {
                                width: 124px;
                            }
                        """.trimIndent()
                    }
                }
            }
            body {
                table(classes = "Root-Table") {
                    tr(classes = "Root-Tr") {
                        td(classes = "Root-Td") {
                            div(classes = "Box") {
                                h1 {
                                    +"Welcome!"
                                }
                                div(classes = "Icon") {
                                    img(
                                        src = "https://raw.gitcode.com/keygenqt_vz/web-api-benchmark/raw/main/kotlin-ktor/assets/kodee.svg",
                                        alt = "Kodee"
                                    )
                                }
                                div {
                                    +"REST API written in Kotlin & Ktor."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
