import { invoke } from "@tauri-apps/api/core";
import type { Scene, SceneCreate, SceneUpdate, PaginatedResult, Tag, TagCreate, Studio, StudioCreate } from "@/types";

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

// Studio API

export async function fetchStudios(): Promise<Studio[]> {
  return invoke("studio_list");
}

export async function fetchStudio(id: number): Promise<Studio | null> {
  return invoke("studio_find", { id });
}

export async function createStudio(input: StudioCreate): Promise<Studio> {
  return invoke("studio_create", { input });
}

export async function deleteStudio(id: number): Promise<boolean> {
  return invoke("studio_destroy", { id });
}
