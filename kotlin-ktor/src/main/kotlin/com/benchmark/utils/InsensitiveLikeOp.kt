package com.benchmark.utils

import org.jetbrains.exposed.v1.core.ComparisonOp
import org.jetbrains.exposed.v1.core.Expression
import org.jetbrains.exposed.v1.core.ExpressionWithColumnType
import org.jetbrains.exposed.v1.core.Op
import org.jetbrains.exposed.v1.core.QueryParameter

// Define a custom Op class for ILIKE
class InsensitiveLikeOp(expr1: Expression<*>, expr2: Expression<*>) : ComparisonOp(expr1, expr2, "ILIKE")

// Create an infix extension function for convenience
infix fun <T : String?> ExpressionWithColumnType<T>.ilike(pattern: T): Op<Boolean> =
    InsensitiveLikeOp(this, QueryParameter(pattern, columnType))
