export class Property {
  equals(_other: Property) {
    return false;
  }
  interpolate(next: typeof this, fac: number) {
    return fac < 0.5 ? this : next;
  }
}


export interface Component<E extends Element> {
  getRoot(): E;
}

export abstract class Element implements Component<Element> {
  getRoot() { return this; }

  provide(): { [key: string]: any } {
    return {};
  }

  inject(_deps: { [key: string]: any }) {}
}


export class Children<E extends Element = Element> {
  arr: Component<E>[] = [];
  adapter?: ChildrenAdapter<E>;
  element: Element;

  constructor(element: Element) {
    this.element = element;
  }

  append(component: Component<E>) {
    component.getRoot().inject(this.element.provide());
    this.arr.push(component);
    this.adapter?.append(component.getRoot());
  }

  get(index: number) {
    return this.arr[index];
  }

  get length() {
    return this.arr.length;
  }

  [Symbol.iterator]() {
    return this.arr[Symbol.iterator]();
  }
}

export interface ChildrenAdapter<E> {
  append(element: E): void;
}
