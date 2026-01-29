use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

use crate::app::app_config::AppConfig;

/// Configure tracing
pub fn configure_tracing(config: &AppConfig) {
    if config.server.development {
        let layer = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into());
        registry().with(layer).with(fmt::layer()).init()
    }
}
