export interface Scene {
  id: number;
  title: string | null;
  details: string | null;
  code: string | null;
  director: string | null;
  url: string | null;
  date: string | null;
  rating: number | null;
  organized: boolean;
  studio_id: number | null;
  resume_time: number;
  play_duration: number;
  created_at: string;
  updated_at: string;
}

export interface SceneCreate {
  title?: string;
  details?: string;
  code?: string;
  director?: string;
  url?: string;
  date?: string;
  rating?: number;
  studio_id?: number;
}

export interface SceneUpdate {
  id: number;
  title?: string;
  details?: string;
  code?: string;
  director?: string;
  url?: string;
  date?: string;
  rating?: number;
  organized?: boolean;
  studio_id?: number;
  resume_time?: number;
}

export interface PaginatedResult<T> {
  items: T[];
  total: number;
  page: number;
  per_page: number;
}
