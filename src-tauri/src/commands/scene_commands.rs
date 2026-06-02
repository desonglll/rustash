use crate::models::app_state::AppState;
use crate::models::scene::{SceneCreate, SceneUpdate};
use crate::services::scene_service;
use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[tauri::command]
pub fn scene_list(
    state: tauri::State<'_, AppState>,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Result<PaginatedResult<crate::models::scene::Scene>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(25);
    let items = scene_service::list(&state.db, page, per_page).map_err(|e| e.to_string())?;
    let total = scene_service::count(&state.db).map_err(|e| e.to_string())?;
    Ok(PaginatedResult { items, total, page, per_page })
}

#[tauri::command]
pub fn scene_find(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<Option<crate::models::scene::Scene>, String> {
    scene_service::find(&state.db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn scene_create(
    state: tauri::State<'_, AppState>,
    input: SceneCreate,
) -> Result<crate::models::scene::Scene, String> {
    scene_service::create(&state.db, &input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn scene_update(
    state: tauri::State<'_, AppState>,
    input: SceneUpdate,
) -> Result<Option<crate::models::scene::Scene>, String> {
    scene_service::update(&state.db, &input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn scene_destroy(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<bool, String> {
    scene_service::destroy(&state.db, id).map_err(|e| e.to_string())
}
