import { Binding } from "./binding";
import { Property } from "./common";


export type BindingSet = {
  [key: string]: Binding | BindingSet;
};

export type PropertySet<B> = {
  [P in keyof B]: B[P] extends Binding ? B[P]["value"] : PropertySet<B[P]>;
};

export class Model<B extends BindingSet> {
  props: PropertySet<B>;

  constructor(public bindings: B) {
    this.props = this._defineProperties(bindings, {}) as PropertySet<B>;
  }

  _defineProperty(bindings: BindingSet, key: string, target: Object) {
    let item: Binding | BindingSet = bindings[key];
    if(item instanceof Binding) {
      Object.defineProperty(target, key, {
        configurable: false,
        enumerable: true,
        get() { return (item as Binding).get(); },
        set(value: Property) { (item as Binding).set(value); },
      });
    } else {
      const value = this._defineProperties(item as BindingSet, {});
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

  _defineProperties(bindings: BindingSet, target: Object) {
    for(const key of Object.keys(bindings)) {
      this._defineProperty(bindings, key, target);
    }
    return target;
  }
}
