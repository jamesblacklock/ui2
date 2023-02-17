import { Property } from "./common";

export class Brush extends Property {
  static get RED() { return new Color(1.0, 0, 0, 1) };
  static get TRANSPARENT() { return new Color(0, 0, 0, 0) };

  static default() { return this.TRANSPARENT as Brush; }

  static rgb(r: number, g: number, b: number) {
    return new Color(r, g, b, 1);
  }

  static rgba(r: number, g: number, b: number, a: number) {
    return new Color(r, g, b, a);
  }

  constructor() {
    super();
  }
}

export class Color extends Brush {
  r: number;
  g: number;
  b: number;
  a: number;

  constructor(r: number, g: number, b: number, a: number) {
    super();
    this.r = Math.max(Math.min(r, 1), 0);
    this.g = Math.max(Math.min(g, 1), 0);
    this.b = Math.max(Math.min(b, 1), 0);
    this.a = Math.max(Math.min(a, 1), 0);
  }

  interpolate(next: Color, fac: number) {
    return new Color(
      (next.r - this.r) * fac + this.r,
      (next.g - this.g) * fac + this.g,
      (next.b - this.b) * fac + this.b,
      (next.a - this.a) * fac + this.a,
    ) as this;
  }
}
