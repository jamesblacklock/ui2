import { Length } from './length';
import { Brush } from './brush';
import { Component, Property, Empty, Collection } from './common';
import { Binding, PropertyConstructor } from './binding';
import { Model } from './model';
import { String, Int, Float } from './types';
import { BindingPreset } from './binding-preset';

export * from './types';
export * from './common';
export * from './length';
export * from './brush';
export * from './transition';
export * from './binding';
export * from './model';
export * as Builtins from './builtins';

export type EventHandler<E> = (el: E, e: Event) => unknown;

export interface EventEmitterAdapter<E> {
  addListener(handler: EventHandler<E>, name: string): void;
  removeListener(name: string): void;
}

export class EventEmitter<E> {
  listeners: { [key: string]: EventHandler<E> } = {};
  adapter?: EventEmitterAdapter<E>;

  constructor(public element: E) {}

  addListener(handler: EventHandler<E>, name: string = 'default') {
    this.listeners[name] = handler;
    this.adapter?.addListener(handler, name);
  }

  removeListener(name: string = 'default') {
    delete this.listeners[name];
    this.adapter?.removeListener(name);
  }
}

function pointerEvents<E>(element: E) {
  return {
    click: new EventEmitter<E>(element),
  }
}



export abstract class Container<E = unknown, Child = unknown> extends Component<E> {
  static default() { return new EmptyContainer(null); }
  static coerce(e: any): Container<unknown> { return e instanceof Container ? e : this.default(); }

  children: Children<Child>;

  constructor(root: Container<E, Child> | null) {
    super();
    this.children = root?.children ?? new Children(this);
  }

  provide(): { [key: string]: any } {
    return {
      parent: new Binding(Container).set(this).freeze(),
    };
  }
}

export class EmptyContainer extends Container {
  getRoots() {
    return [];
  }
}

export interface ChildrenAdapter {
  append(elements: any): void;
  replace(cur: any, next: any): void;
}

export class Children<T = unknown> {
  arr: [any, T[]][] = [];
  adapter?: ChildrenAdapter;
  parent: Container<unknown>;
  childCount = new Binding(Int);

  constructor(parent: any) {
    this.parent = parent as Container<Object>;
  }

  append(child: Component<T>) {
    this.childCount.set(Int.from(this.childCount.get().value + 1))
    child.inject(this.parent.provide());
    const roots = child.getRoots();
    if(roots.length === 0) {
      roots.push({} as T);
    }
    this.arr.push([child, roots]);
    this.adapter?.append(roots);
  }

  update(child: Component<T>) {
    const result = this.arr.find(e => e[0] === child);
    if(!result) {
      console.warn("tried to replace component not found in container");
      return;
    }
    const prevRoots = result[1];
    const nextRoots = child.getRoots();
    if(nextRoots.length === 0) {
      nextRoots.push({} as T);
    }
    result[1] = nextRoots;
    this.adapter?.replace(prevRoots, nextRoots);
  }

  get(index: number) {
    return this.arr[index]?.[0];
  }

  get length() {
    return this.arr.length;
  }

  *[Symbol.iterator](): Generator<T> {
    for(const [c] of this.arr) {
      yield c;
    }
  }
}

export class Text extends Component<Text> {
  #t: 'Text' = 'Text';
  #model = new Model({
    content: new Binding(String),
  });

  get bindings() {
    return this.#model.bindings;
  }

  get props() {
    return this.#model.props;
  }

  getRoots() { return [this]; }
}

export type FrameSize = {
  width: Binding<Length>,
  height: Binding<Length>,
}

export class Rect extends Container<Rect> {
  #t: 'Rect' = 'Rect';
  #model = new Model({
    fill: new Binding(Brush),
    x1: new Binding(Length),
    y1: new Binding(Length),
    x2: new Binding(Length),
    y2: new Binding(Length),
    width: new Binding(Length),
    height: new Binding(Length),
  });
  protected privateModel = new Model({
    parentSize: {
      width: new Binding(Length),
      height: new Binding(Length),
    }
  });

  scaleToParent = new BindingPreset(Float)
    .addChild(
      this.bindings.x1,
      [this.privateModel.bindings.parentSize.width] as const,
      (value, [width]) => width.mul(value.value/2).neg(),
    )
    .addChild(
      this.bindings.x2,
      [this.privateModel.bindings.parentSize.width] as const,
      (value, [width]) => width.mul(value.value/2),
    )
    .addChild(
      this.bindings.y1,
      [this.privateModel.bindings.parentSize.height] as const,
      (value, [height]) => height.mul(value.value/2).neg(),
    )
    .addChild(
      this.bindings.y2,
      [this.privateModel.bindings.parentSize.height] as const,
      (value, [height]) => height.mul(value.value/2),
    );;

  readonly events = {
    pointer: pointerEvents(this as Rect),
  };

  constructor() {
    super(null);
    this.bindings.width.connect(
      [this.bindings.x1, this.bindings.x2] as const,
      ([x1, x2]) => x2.sub(x1)
    ).freeze();
    this.bindings.height.connect(
      [this.bindings.y1, this.bindings.y2] as const,
      ([y1, y2]) => y2.sub(y1)
    ).freeze();
  }

  get bindings() {
    return this.#model.bindings;
  }

  get props() {
    return this.#model.props;
  }

  provide() {
    return {
      ...super.provide(),
      frameSize: { width: this.bindings.width, height: this.bindings.height },
    };
  }
  inject(deps: { [key: string]: any; }) {
    super.inject(deps);
    if(deps.frameSize) {
      this.privateModel.bindings.parentSize.width.connect([deps.frameSize.width]);
      this.privateModel.bindings.parentSize.height.connect([deps.frameSize.height]);
    }
  }

  getRoots() { return [this]; }
}

export class Slot<T = unknown> extends Component<T> {
  #t: 'Slot' = 'Slot';
  #model = new Model({
    component: new Binding(Component),
  });

  parent = new Binding(Container) as Binding<Container<unknown, T>>;
  injected: { [key: string]: any; } = {};
  parentHasBeenSet = false;

  constructor() {
    super();
    this.bindings.component.onChange(this.updateComponent.bind(this));
  }

  inject(deps: { [key: string]: any; }) {
    this.parent.set(deps.parent.get());
    this.parentHasBeenSet = true;
    this.injected = deps;
    this.updateComponent(this.props.component, this.props.component);
  }

  get bindings() {
    return this.#model.bindings;
  }

  get props() {
    return this.#model.props;
  }

  getRoots(): T[] {
    return this.props.component.getRoots();
  }

  private updateComponent(prev: Component, cur: Component) {
    cur.inject(this.injected);
    if(this.parentHasBeenSet && prev !== cur) {
      this.parent.get().children.update(this);
    }
  }
}

class LayoutEnum implements Property {
  static Row = new LayoutEnum();
  static Column = new LayoutEnum();
  static default() { return this.Row; }
  static coerce(e: any) { return e instanceof this ? e : this.Row; }
  constructor() {
    if(LayoutEnum.Column !== undefined) {
      throw new Error('cannot construct new instances of Enum.Layout');
    }
  }
  interpolate(next: this, _fac: number) {
    return next;
  }
  equals(other: this): boolean {
    return other === this;
  }
}

export const Enum = {
  Layout: LayoutEnum,
};

export class Pane extends Container<Pane> {
  #t: 'Pane' = 'Pane';
  #privateModel = new Model({
    siblingCount: new Binding(Int),
    padding: new Binding(Length),
    selfSize: {
      width: new Binding(Length),
      height: new Binding(Length),
    },
  });
  constructor() {
    super(null);
  }

  getRoots() { return [this]; }

  inject(deps: { [key: string]: any; }): void {
    if(deps.layoutInfo) {
      this.#privateModel.bindings.siblingCount.connect([deps.layoutInfo.itemCount]);
      this.#privateModel.bindings.padding.connect([deps.layoutInfo.padding]);
    }
    if(deps.frameSize) {
      this.#privateModel.bindings.selfSize.width.connect(
        [
          deps.frameSize.width as Binding<Length>,
          this.#privateModel.bindings.padding,
          this.#privateModel.bindings.siblingCount,
        ] as const,
        ([width, padding, siblingCount]) => {
          return width.sub(padding.mul(2)).div(siblingCount.value);
        },
      );
      this.#privateModel.bindings.selfSize.height.connect(
        [
          deps.frameSize.height as Binding<Length>,
          this.#privateModel.bindings.padding,
          this.#privateModel.bindings.siblingCount,
        ] as const,
        ([height, padding, siblingCount]) => height.sub(padding.mul(2)),
      );
    }
  }
  provide() {
    return {
      ...super.provide(),
      frameSize: this.#privateModel.bindings.selfSize,
      layoutInfo: {
        itemCount: this.children.childCount,
      },
    };
  }
}

export class Layout extends Container<Layout, Pane> {
  #t: 'Layout' = 'Layout';
  #publicModel = new Model({
    padding: new Binding(Length),
    layout: new Binding(Enum.Layout),
  });
  protected privateModel = new Model({
    parentSize: {
      width: new Binding(Length),
      height: new Binding(Length),
    },
  });

  constructor() {
    super(null);
  }

  get bindings() {
    return this.#publicModel.bindings;
  }

  get props() {
    return this.#publicModel.props;
  }

  inject(deps: { [key: string]: any }) {
    if(deps.frameSize) {
      this.privateModel.bindings.parentSize.width.connect([deps.frameSize.width]);
      this.privateModel.bindings.parentSize.height.connect([deps.frameSize.height]);
    }
  }
  provide() {
    return {
      ...super.provide(),
      frameSize: this.privateModel.bindings.parentSize,
      layoutInfo: {
        itemCount: this.children.childCount,
        padding: this.bindings.padding,
      },
    };
  }

  getRoots() { return [this]; }
}

export class Repeater<P extends Property = Property, E = unknown> extends Component<E> {
  #t : 'Repeater' = 'Repeater';
  #model;
  components: Component<E>[] = [];
  parent = new Binding(Container);
  // parentHasBeenSet = false;

  constructor(init: PropertyConstructor<Collection<P>>, private proc: (p: P) => Component<E>[]) {
    super();
    this.#model = new Model({
      collection: new Binding(init),
    });

    this.bindings.collection.onChange(() => this._updateItems());
  }

  get bindings() {
    return this.#model.bindings;
  }

  get props() {
    return this.#model.props;
  }

  inject(deps: { [key: string]: any; }): void {
    this.parent.set(deps.parent.get());
    // this.parentHasBeenSet = true;
    for(const item of this.components) {
      item.inject(deps);
    }
  }

  getRoots() {
    return this.components.map(e => e.getRoots()).flat();
  }

  _updateItems() {
    const all = [];
    for(const p of this.props.collection.iter()) {
      all.push(this.proc.call(null, p));
    }
    this.components = all.flat();
    this.parent.get().children.update(this);
  }
}

export interface Dom {
  Rect(): Rect;
  Slot<T>(): Slot<T>;
  Text(): Text;
  Layout(): Layout;
  Pane(): Pane;
  Empty(): Empty;
  Repeater<P extends Property, E = unknown>(
    init: PropertyConstructor<Collection<P>>,
    proc: (p: P) => Component<E>[],
  ): Repeater<P, E>;
}
