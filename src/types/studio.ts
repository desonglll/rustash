export interface Studio {
  id: number;
  name: string;
  url: string | null;
  parent_id: number | null;
  details: string | null;
  rating: number | null;
  ignore_auto_tag: boolean;
  favorite: boolean;
  organized: boolean;
  created_at: string;
  updated_at: string;
}

export interface StudioCreate {
  name: string;
  url?: string;
  parent_id?: number;
  details?: string;
  rating?: number;
}
