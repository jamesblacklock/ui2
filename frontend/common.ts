export interface Property {
  equals(other: this): boolean;
  interpolate(next: this, fac: number): this;
}

export abstract class Component<E = unknown> implements Property {
  static default() { return new Empty() };
  static coerce(e: any) { return e instanceof Component ? e : new Empty() };
  equals(other: this) { return this === other }
  interpolate(next: this, _fac: number) { return next }
  inject(_deps: { [key: string]: any }) {}
  abstract getRoot(): E;
}

export class Empty extends Component {
  getRoot() { return this; }
}
