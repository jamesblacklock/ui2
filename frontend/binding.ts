import { Property } from './common';
import { Transition } from './transition';

export type Transformer<P extends Property, A extends any[] = Property[]> = (p: A) => P;
export type PropertyConstructor<P extends Property> = { default: () => P, coerce: (e: any) => P };

export class Binding<P extends Property = Property> {
  prevValue?: P;
  value: P;
  transitionStartTime = 0;
  transition?: Transition;
  notify?: (prev: P, cur: P) => unknown;
  descendants: Set<Binding<P>>;
  ancestors: Binding<P>[] = [];
  transform: Transformer<P> = p => p[0] as P;
  isComputed = false;
  readonly = false;

  constructor(private init: PropertyConstructor<P>, transition?: Transition) {
    this.transition = Object.freeze(transition);
    this.descendants = new Set();
    this.value = init.default();
  }

  onChange(notify: (prev: P, cur: P) => unknown) {
    this.notify = notify;
    return this;
  }

  get(interpolate = false): P {
    if(!interpolate || !this.transition || !this.prevValue) {
      return this.value;
    }
    const fac = this.transition.interpolate(this.transitionStartTime);
    return this.prevValue.interpolate(this.value, fac);
  }

  set(value: P) {
    if(this.readonly) {
      throw new Error('cannot rebind: the binding is readonly');
    }
    this._setInternal(value);
    return this;
  }

  freeze() {
    this.readonly = true;
    return this;
  }

  private _setInternal(value: P, isComputedValue = false) {
    if(this.isComputed && !isComputedValue) {
      throw new Error('cannot directly set descendant binding!');
    }

    this.prevValue = this.get(true);
    this.value = this.init.coerce(value);
    this.transitionStartTime = Date.now();

    this.notify?.(this.prevValue, this.value);
    for(const binding of this.descendants) {
      binding._update();
    }
  }

  disconnect() {
    if(this.readonly) {
      throw new Error('cannot rebind: the binding is readonly');
    }
    for(const ancestor of this.ancestors) {
      ancestor.descendants.delete(this);
    }
    this.ancestors = [];
    this.isComputed = false;
  }

  connect<
    A extends any[],
    AP extends any[] = { [P in keyof A]: A[P]['value'] }
  >(ancestors: A, transform: Transformer<P, AP> = p => p[0] as P, transition?: Transition) {
    this.disconnect();

    this.ancestors = ancestors;
    this.transform = transform as Transformer<P>;
    this.transition = transition;
    this.isComputed = true;

    for(const ancestor of this.ancestors) {
      ancestor.descendants.add(this);
    }

    this._update();

    return this;
  }

  _update() {
    const value = this.transform(this.ancestors.map(b => b.get()));
    this._setInternal(value, true);
  }
}
