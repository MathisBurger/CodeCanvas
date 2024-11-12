export interface SpotlightGroup {
  id: number;
  title: string;
}

export interface SpotlightAssignment {
  id: number;
  title: string;
  group_id: number;
}

export interface Spotlight3Response {
  groups: SpotlightGroup[];
  assignments: SpotlightAssignment[];
}
