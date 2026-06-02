use crate::models::app_state::AppState;
use crate::models::performer::PerformerCreate;
use crate::services::performer_service;
use crate::commands::scene_commands::PaginatedResult;

#[tauri::command]
pub fn performer_list(
    state: tauri::State<'_, AppState>,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Result<PaginatedResult<crate::models::performer::Performer>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(25);
    let items = performer_service::list(&state.db, page, per_page).map_err(|e| e.to_string())?;
    let total = performer_service::count(&state.db).map_err(|e| e.to_string())?;
    Ok(PaginatedResult { items, total, page, per_page })
}

#[tauri::command]
pub fn performer_find(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<Option<crate::models::performer::Performer>, String> {
    performer_service::find(&state.db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn performer_create(
    state: tauri::State<'_, AppState>,
    input: PerformerCreate,
) -> Result<crate::models::performer::Performer, String> {
    performer_service::create(&state.db, &input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn performer_destroy(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<bool, String> {
    performer_service::destroy(&state.db, id).map_err(|e| e.to_string())
}
