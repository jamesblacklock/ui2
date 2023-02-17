import { Property } from './common';
import { Transition } from './transition';

type Transformer<P extends Property> = (p: P[]) => P;
type PropertyConstructor<P extends Property> = { default: () => P };

export class Binding<P extends Property = Property> implements Binding<P> {
  prevValue?: P;
  value: P;
  transitionStartTime = 0;
  transition?: Transition;
  notify?: () => unknown;
  descendants: Set<Binding<P>>;
  ancestors: Binding<P>[] = [];
  isComputed = false;
  transform: Transformer<P> = p => p[0];

  constructor(init: P | PropertyConstructor<P>, transition?: Transition) {
    let value;
    if(init instanceof Function) {
      value = (init as PropertyConstructor<P>).default();
    } else {
      value = init as P;
    }
    this.transition = Object.freeze(transition);
    this.descendants = new Set();
    this.value = Object.freeze(value);
  }

  onChange(notify: () => unknown) {
    this.notify = notify;
    return this;
  }

  get(interpolate = false): P {
    // if(pull && this.ancestors.length > 0) {
    //   this.prevValue = this.value;
    //   this.value = this.transform(this.ancestors.map(b => b.get(false, true)));
    // }

    if(!interpolate || !this.transition || !this.prevValue) {
      return this.value;
    }
    const fac = this.transition.interpolate(this.transitionStartTime);
    return this.prevValue.interpolate(this.value, fac);
  }

  set(value: P) {
    this._setInternal(value);
  }

  _setInternal(value: P, isComputedValue = false) {
    if(this.isComputed && !isComputedValue) {
      throw new Error('cannot directly set descendant binding!');
    }

    this.prevValue = this.get(true);
    this.value = value;
    this.transitionStartTime = Date.now();

    this.notify?.();
    for(const binding of this.descendants) {
      binding._update();
    }
  }

  disconnect() {
    for(const ancestor of this.ancestors) {
      ancestor.descendants.delete(this);
    }
    this.ancestors = [];
    this.isComputed = false;
  }

  connect(ancestors: Binding<P>[], transform: Transformer<P> = p => p[0], transition?: Transition) {
    this.disconnect();

    this.ancestors = ancestors;
    this.transform = transform;
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
