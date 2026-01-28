plugins {
    alias(libs.plugins.kotlin.serialization)
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.ktor)
}

group = "com.benchmark"
version = "0.0.1"

application {
    mainClass.set("$group.MainKt")
}

ktor {
    fatJar {
        archiveFileName.set("kotlin-ktor.jar")
    }
}

// https://github.com/flyway/flyway/issues/4170
tasks {
    shadowJar {
        mergeServiceFiles {
            include("META-INF/services/**")
            duplicatesStrategy = DuplicatesStrategy.INCLUDE
        }
    }
}

repositories {
    mavenCentral()
}

dependencies {
    // Ktor
    implementation(libs.ktor.server.core)
    implementation(libs.ktor.server.netty)
    implementation(libs.ktor.server.auth)
    implementation(libs.ktor.server.sessions)
    implementation(libs.ktor.server.status.pages)
    implementation(libs.ktor.server.content.negotiation)
    implementation(libs.ktor.server.content.json)
    implementation(libs.ktor.server.call.logging)

    // DB
    implementation(libs.postgresql)
    implementation(libs.exposed.core)
    implementation(libs.exposed.jdbc)
    implementation(libs.exposed.dao)
    implementation(libs.exposed.time)

    // Migration
    implementation(libs.flyway.core)
    implementation(libs.flyway.postgresql)

    // DI
    implementation(libs.koin.core)
    implementation(libs.koin.ktor)

    // HOCON
    implementation(libs.serialization.hocon)
    implementation(libs.config.hocon)

    // security
    implementation(libs.security.crypto)
    implementation(libs.security.logging)
    implementation(libs.security.jdk18on)
    implementation(libs.security.codec)

    // Other
    implementation(libs.logback.classic)
    implementation(libs.picocli)
}
