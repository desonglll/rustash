import { invoke } from "@tauri-apps/api/core";
import type { Scene, SceneCreate, SceneUpdate, PaginatedResult } from "@/types";

export async function fetchScenes(
  page = 1,
  perPage = 25
): Promise<PaginatedResult<Scene>> {
  return invoke("scene_list", { page, perPage });
}

export async function fetchScene(id: number): Promise<Scene | null> {
  return invoke("scene_find", { id });
}

export async function createScene(input: SceneCreate): Promise<Scene> {
  return invoke("scene_create", { input });
}

export async function updateScene(input: SceneUpdate): Promise<Scene | null> {
  return invoke("scene_update", { input });
}

export async function deleteScene(id: number): Promise<boolean> {
  return invoke("scene_destroy", { id });
}
