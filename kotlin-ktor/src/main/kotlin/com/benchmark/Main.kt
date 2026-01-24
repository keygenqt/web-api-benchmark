package com.benchmark

import com.benchmark.cli.AppCLI
import picocli.CommandLine

/**
 * Entry point to the application.
 */
fun main(args: Array<String>) {
    CommandLine(AppCLI(args)).execute(*args)
}
