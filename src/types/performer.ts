export interface Performer {
  id: number;
  name: string;
  disambiguation: string | null;
  gender: string | null;
  url: string | null;
  birthdate: string | null;
  ethnicity: string | null;
  country: string | null;
  eye_color: string | null;
  height: number | null;
  measurements: string | null;
  fake_tits: string | null;
  tattoos: string | null;
  piercings: string | null;
  favorite: boolean;
  career_length: string | null;
  details: string | null;
  death_date: string | null;
  hair_color: string | null;
  weight: number | null;
  rating: number | null;
  ignore_auto_tag: boolean;
  created_at: string;
  updated_at: string;
}

export interface PerformerCreate {
  name: string;
  disambiguation?: string;
  gender?: string;
  url?: string;
  birthdate?: string;
  ethnicity?: string;
  country?: string;
  eye_color?: string;
  height?: number;
  measurements?: string;
  fake_tits?: string;
  tattoos?: string;
  piercings?: string;
  career_length?: string;
  details?: string;
  death_date?: string;
  hair_color?: string;
  weight?: number;
}
