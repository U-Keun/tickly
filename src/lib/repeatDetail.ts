export function parseRepeatDetail(repeatDetail: string | null | undefined): number[] {
  if (!repeatDetail) return [];

  try {
    const parsed = JSON.parse(repeatDetail);
    if (!Array.isArray(parsed)) return [];

    return parsed
      .map((value) => Number(value))
      .filter((value) => Number.isInteger(value));
  } catch {
    return [];
  }
}

export function stringifyRepeatDetail(repeatDetail: number[]): string | null {
  return repeatDetail.length > 0 ? JSON.stringify(repeatDetail) : null;
}
