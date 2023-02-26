import { Collection, Property, PropertyConstructor } from "./common";

abstract class ScalarValueProperty<T> implements Property {
  constructor(readonly value: T) {}
  equals(other: this) {
    return other instanceof ScalarValueProperty && this.value === other.value;
  }
  abstract interpolate(next: this, fac: number): this
  valueOf() { return this.value; }
}

export class Int extends ScalarValueProperty<number> implements Collection<Int> {
  static from(value: number) { return new Int(value); };
  static default() { return new this(0); }
  static coerce(e: any) {
    if(e instanceof Int) {
      return e;
    } else {
      const f = Float.coerce(e);
      return Int.from(f.value);
    }
  }
  constructor(value: number) { super(value << 0); }
  interpolate(next: Int, fac: number) {
    return Int.from((next.value - this.value) * fac + this.value) as this;
  }
  *iter() {
    for(let i = 1; i <= this.value; i++) {
      yield Int.from(i);
    }
  }
}

export class Float extends ScalarValueProperty<number> {
  static from(value: number) { return new this(value); };
  static default() { return new Float(0); }
  static coerce(e: any) {
    if(e instanceof Float) {
      return e;
    } else if(e instanceof Int) {
      return Float.from(e.value);
    } else if(e instanceof Boolean) {
      return Float.from(e.value ? 1 : 0);
    } else if(typeof e === 'number') {
      return Float.from(e);
    } else {
      const n = parseFloat(globalThis.String(e));
      return Float.from(isNaN(n) ? 0 : n);
    }
  }
  constructor(value: number) { super(value); }
  interpolate(next: Float, fac: number) {
    return Float.from((next.value - this.value) * fac + this.value) as this;
  }
}

export class Boolean extends ScalarValueProperty<boolean> {
  static true = new Boolean(true);
  static false = new Boolean(false);
  static from(value: boolean) { return value ? this.true : this.false; };
  static default() { return this.false; }
  static coerce(e: any) {
    if(e instanceof Boolean) {
      return e;
    } else if(e instanceof Int || e instanceof Float) {
      return Boolean.from(!!e.value);
    } else {
      return Boolean.from(!!e);
    }
  }
  constructor(value: boolean) {
    if(Boolean.false !== undefined) {
      throw new Error('cannot construct new instances of Boolean');
    }
    super(value);
  }
  interpolate(next: this, _fac: number): this {
      return next;
  }
}

export class String extends ScalarValueProperty<string> {
  static from(value: string) { return new String(value); };
  static default() { return new this(""); }
  static coerce(e: any) {
    if(e instanceof String) {
      return e;
    } else if(e instanceof Int || e instanceof Float || e instanceof Boolean) {
      return String.from(globalThis.String(e.value));
    } else if(typeof e === 'string') {
      return String.from(e);
    } else {
      return String.default();
    }
  }
  constructor(value: string) { super(value); }
  interpolate(next: this, _fac: number): this {
      return next;
  }
}

export class Iter<T extends Property> implements Collection<T> {
  static of<P extends Property>(init: PropertyConstructor<P>): PropertyConstructor<Collection<P>> {
    function defaultFn(): Collection<P> {
      return new Iter(init);
    }

    function coerceFn(e: any): Collection<P> {
      if(typeof e?.constructor?.prototype.iter === 'function') {
        return e;
      } else if(e?.[Symbol.iterator]) {
        const arr: P[] = [];
        for(const item of e?.[Symbol.iterator]()) {
          arr.push(init.coerce(item));
        }
        return List.from(arr);
      } else {
        return defaultFn();
      }
    }

    return { default: defaultFn, coerce: coerceFn };
  }
  constructor(public init: PropertyConstructor<T>) {}
  equals(other: this) {
    return this === other;
  }
  interpolate(next: this, _fac: number): this {
    return next;
  }
  *iter() {}
}

export class List<T extends Property> implements Collection<T> {
  static from<P extends Property>(value: P[]): List<P> { return new List(value); };
  constructor(public value: T[] = []) {}
  equals(other: this) {
    return this === other;
  }
  interpolate(next: this, _fac: number): this {
    return next;
  }
  *iter() {
    for(const item of this.value) {
      yield item;
    }
  }
}
