export interface Property {
  equals(other: this): boolean;
  interpolate(next: this, fac: number): this;
}

export interface Collection<P extends Property> extends Property {
  iter(): Generator<P>;
}

export abstract class Component<E = unknown> implements Property {
  static default() { return new NullComponent() };
  static coerce(e: any) { return e instanceof Component ? e : new NullComponent() };
  equals(other: this) { return this === other }
  interpolate(next: this, _fac: number) { return next }
  inject(_deps: { [key: string]: any }) {}
  abstract getRoots(): E[];
}

export class NullComponent extends Component {
  getRoots() { return []; }
}
