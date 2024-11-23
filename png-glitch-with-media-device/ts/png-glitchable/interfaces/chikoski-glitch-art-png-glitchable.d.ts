export namespace ChikoskiGlitchArtPngGlitchable {
  export { ScanLine };
  export { Png };
}
/**
 * # Variants
 * 
 * ## `"none"`
 * 
 * ## `"sub"`
 * 
 * ## `"up"`
 * 
 * ## `"average"`
 * 
 * ## `"paeth"`
 */
export type FilterType = 'none' | 'sub' | 'up' | 'average' | 'paeth';

export class Png {
  getScanLines(): ScanLine[];
  read(): Uint8Array;
  static create(data: Uint8Array): Png;
}

export class ScanLine {
  getFilterType(): FilterType;
  setFilterType(t: FilterType): void;
  size(): number;
  getPixelAt(index: number): number;
  setPixelAt(index: number, value: number): void;
  read(): Uint8Array;
  write(pixels: Uint8Array): void;
}
