import { Brush as BrushType } from "./brush";

export const Math: { [key: string]: any } = {
  random: globalThis.Math.random,
};
export const Brush = {
  rgb: BrushType.rgb,
  rgba: BrushType.rgba,
};
