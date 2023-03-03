import { ComponentProperty } from './dom';
import { Property } from './runtime';

export type PropertySet = {
  [key: string]: Property<any> | PropertySet;
};

export type ValueSet<B> = {
  [P in keyof B]: B[P] extends Property<infer Q> ? Q : ValueSet<B[P]>;
};

export class Model<B extends PropertySet> {
  props: ValueSet<B>;

  constructor(public bindings: B) {
    this.props = this._defineProperties(bindings, {}) as ValueSet<B>;
  }

  _defineProperty(bindings: PropertySet, key: string, target: Object) {
    let item: Property | PropertySet = bindings[key];
    if(item instanceof Property || item instanceof ComponentProperty) {
      Object.defineProperty(target, key, {
        configurable: false,
        enumerable: true,
        get() { return (item as Property).get(); },
        set(value) { (item as Property).set(value); },
      });
    } else {
      const value = this._defineProperties(item as PropertySet, {});
      Object.defineProperty(target, key, {
        configurable: false,
        enumerable: true,
        get() { return value; },
        set(newValue) {
          for(const [k, v] of Object.entries(newValue)) {
            if(k in value) {
              (value as any)[k] = v;
            }
          }
        }
      });
    }
  }

  _defineProperties(bindings: PropertySet, target: Object) {
    for(const key of Object.keys(bindings)) {
      this._defineProperty(bindings, key, target);
    }
    return target;
  }
}
