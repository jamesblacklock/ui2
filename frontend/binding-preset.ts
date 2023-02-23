import { Binding, PropertyConstructor, Transformer } from "./binding";
import { Property } from "./common";

type BindingPresetChild = {
  binding: Binding<any>;
  ancestors: Binding[];
  transform: Transformer<Property>;
};

export class BindingPreset<T extends Property> {
  binding: Binding<T>;
  #enabled = false;
  children: BindingPresetChild[] = [];

  constructor(init: PropertyConstructor<T>) {
    this.binding = new Binding(init);
  }

  get(): T|undefined {
    return this.binding?.get();
  }

  set(value: T) {
    this.enabled = true;
    this.binding.set(value);
  }

  connect(ancestors: Binding<T>[], transform?: Transformer<T>) {
    this.enabled = true;
    this.binding.connect(ancestors, transform);
  }

  disconnect() {
    this.binding.disconnect()
  }

  get enabled() { return this.#enabled; }
  set enabled(value: boolean) {
    if(this.#enabled === value) {
      return;
    }
    this.#enabled = value;
    const f: (child: BindingPresetChild) => void =
      value ? child => this._connectChild(child) : child => this._disconnectChild(child);

    for(const child of this.children) {
      f(child);
    }
  }

  _connectChild(child: BindingPresetChild) {
    child.binding.connect([this.binding, ...child.ancestors], child.transform);
  }

  _disconnectChild(child: BindingPresetChild) {
    child.binding.disconnect();
  }

  addChild<P extends Property, A extends any[], AP = { [P in keyof A]: A[P]['value'] }>(
    binding: Binding<P>,
    ancestors: A,
    transform: (value: T, properties: AP) => P,
  ) {
    const mappedTransform: Transformer<P> = p => transform(p[0] as T, p.slice(1) as AP);
    const child: BindingPresetChild = { binding, ancestors, transform: mappedTransform };
    this.children.push(child);
    if(this.enabled) {
      this._connectChild(child);
    }
    return this;
  }
}
