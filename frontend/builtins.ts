import { Brush as BrushType } from "./runtime";

export const Math: { [key: string]: any } = {
  random: globalThis.Math.random,
};
export const Brush = {
  rgb: (r: number, g: number, b: number) => BrushType.rgba(r, g, b, 1),
  rgba: BrushType.rgba,
};
