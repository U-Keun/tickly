export interface Category {
  id: number;
  name: string;
  display_order: number;
}

export interface TodoItem {
  id: number;
  text: string;
  done: boolean;
  category_id: number | null;
  display_order: number;
  memo: string | null;
}

export interface ThemeColors {
  paper: string;
  canvas: string;
  mist: string;
  stroke: string;
  ink: string;
  inkMuted: string;
  accentSky: string;
  accentSkyStrong: string;
  accentMint: string;
  accentMintStrong: string;
  accentPeach: string;
  accentPeachStrong: string;
  white: string;
  border: string;
}

export interface ThemePreset {
  id: string;
  name: string;
  colors: ThemeColors;
}
