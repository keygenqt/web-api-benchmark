use clap::builder::Styles;
use colored::Colorize;

pub fn version() -> &'static str {
    "0.0.1"
}

pub fn about() -> String {
    format!(
        "{} - Axum REST API Demo Application.",
        "Rust Axum".bright_green().bold()
    )
}

pub fn styles() -> Styles {
    Styles::styled()
        .header(clap_cargo::style::HEADER)
        .usage(clap_cargo::style::USAGE)
        .literal(clap_cargo::style::LITERAL)
        .placeholder(clap_cargo::style::PLACEHOLDER)
        .error(clap_cargo::style::ERROR)
        .valid(clap_cargo::style::VALID)
        .invalid(clap_cargo::style::INVALID)
}
