use crate::models::app_state::AppState;
use crate::models::tag::TagCreate;
use crate::services::tag_service;

#[tauri::command]
pub fn tag_list(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::models::tag::Tag>, String> {
    tag_service::list(&state.db).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tag_find(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<Option<crate::models::tag::Tag>, String> {
    tag_service::find(&state.db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tag_create(
    state: tauri::State<'_, AppState>,
    input: TagCreate,
) -> Result<crate::models::tag::Tag, String> {
    tag_service::create(&state.db, &input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn tag_destroy(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<bool, String> {
    tag_service::destroy(&state.db, id).map_err(|e| e.to_string())
}
