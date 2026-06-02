import { invoke } from "@tauri-apps/api/core";
import type { Scene, SceneCreate, SceneUpdate, PaginatedResult, Tag, TagCreate, Performer, PerformerCreate } from "@/types";

// Scene API

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

// Tag API

export async function fetchTags(): Promise<Tag[]> {
  return invoke("tag_list");
}

export async function fetchTag(id: number): Promise<Tag | null> {
  return invoke("tag_find", { id });
}

export async function createTag(input: TagCreate): Promise<Tag> {
  return invoke("tag_create", { input });
}

export async function deleteTag(id: number): Promise<boolean> {
  return invoke("tag_destroy", { id });
}

// Performer API

export async function fetchPerformers(
  page = 1,
  perPage = 25
): Promise<PaginatedResult<Performer>> {
  return invoke("performer_list", { page, perPage });
}

export async function fetchPerformer(id: number): Promise<Performer | null> {
  return invoke("performer_find", { id });
}

export async function createPerformer(input: PerformerCreate): Promise<Performer> {
  return invoke("performer_create", { input });
}

export async function deletePerformer(id: number): Promise<boolean> {
  return invoke("performer_destroy", { id });
}
