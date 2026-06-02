export interface Tag {
  id: number;
  name: string;
  description: string | null;
  ignore_auto_tag: boolean;
  favorite: boolean;
  created_at: string;
  updated_at: string;
}

export interface TagCreate {
  name: string;
  description?: string;
  ignore_auto_tag?: boolean;
  favorite?: boolean;
}
