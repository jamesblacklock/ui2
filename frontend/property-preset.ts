import { Property, Transformer} from "./runtime";

type PropertyPresetChild<P, V> = {
  property: Property<any>;
  parents: readonly Property[];
  transform: Transformer<P, V>;
};

export class PropertyPreset<V> {
  property: Property<V>;
  #enabled = false;
  children: PropertyPresetChild<any, any>[] = [];

  constructor(property: Property<V>) {
    this.property = property;
  }

  get(): V|undefined {
    return this.property?.get();
  }

  set(value: V) {
    this.enabled = true;
    this.property.set(value);
  }

  connect<P extends readonly Property<any>[]>(parents: P, transform: Transformer<P, V>) {
    this.enabled = true;
    this.property.bind(parents, transform);
  }

  disconnect() {
    this.property.unbind()
  }

  get enabled() { return this.#enabled; }
  set enabled(value: boolean) {
    if(this.#enabled === value) {
      return;
    }
    this.#enabled = value;
    const f: (child: PropertyPresetChild<any, V>) => void =
      value ? child => this._bindChild(child) : child => this._unbindChild(child);

    for(const child of this.children) {
      f(child);
    }
  }

  _bindChild(child: PropertyPresetChild<any, any>) {
    child.property.bind([this.property, ...child.parents], child.transform);
  }

  _unbindChild(child: PropertyPresetChild<any, any>) {
    child.property.unbind();
  }

  addChild<P extends readonly Property<any>[], W>(
    property: Property<W>,
    parents: P,
    transform: Transformer<[Property<V>, ...P], W>,
  ) {
    const child: PropertyPresetChild<[Property<V>, ...P], W> = { property, parents, transform };
    this.children.push(child);
    if(this.enabled) {
      this._bindChild(child);
    }
    return this;
  }
}
