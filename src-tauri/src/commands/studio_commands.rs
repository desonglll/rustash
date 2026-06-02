use crate::models::app_state::AppState;
use crate::models::studio::StudioCreate;
use crate::services::studio_service;

#[tauri::command]
pub fn studio_list(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::models::studio::Studio>, String> {
    studio_service::list(&state.db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn studio_find(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<Option<crate::models::studio::Studio>, String> {
    studio_service::find(&state.db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn studio_create(
    state: tauri::State<'_, AppState>,
    input: StudioCreate,
) -> Result<crate::models::studio::Studio, String> {
    studio_service::create(&state.db, &input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn studio_destroy(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<bool, String> {
    studio_service::destroy(&state.db, id).map_err(|e| e.to_string())
}
