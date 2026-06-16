// ©Moksh Jain | 2026
// state.rs
//
// Shared application state for Axum handlers
//
// AppState holds the SQLx PgPool for database access, an
// Arc-wrapped AnalysisEngine for binary analysis, and an
// Arc-wrapped AppConfig. Derives Clone for Axum's State
// extractor.
//
// Connects to:
//   main.rs   - constructed at startup
//   config.rs - AppConfig
//   routes/   - extracted via State<AppState>

use std::sync::Arc;

use aegis-bat_engine::AnalysisEngine;
use sqlx::PgPool;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub engine: Arc<AnalysisEngine>,
    #[allow(dead_code)]
    pub config: Arc<AppConfig>,
}
