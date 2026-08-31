export const SEEDS_MODES = [
  "Light",
  "Dark",
  "Sakura",
  "Momiji",
  "NatureLaw",
  "Disaster",
] as const;

export type SeedsMode = (typeof SEEDS_MODES)[number];

export const DEFAULT_SEEDS_MODE: SeedsMode = "Light";
