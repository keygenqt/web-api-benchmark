package com.benchmark.utils

import kotlin.random.Random

object Password {
    fun generateRandomPassword(length: Int = 12): String {
        val uppercase = ('A'..'Z')
        val lowercase = ('a'..'z')
        val digits = ('0'..'9')
        val allChars = uppercase + lowercase + digits

        // Use a cryptographically strong random number generator for security
        val secureRandom = Random.Default

        return List(length) {
            allChars.random(secureRandom)
        }.joinToString("")
    }
}
