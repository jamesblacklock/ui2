import { PropertyFactory, Length, Property, Transformer, Value } from './runtime';
import { Model } from './model';
import { PropertyPreset } from './property-preset';

export * from './transition';
export * from './model';
export * as Builtins from './builtins';
export * as Coerce from './coerce';
export * from './runtime';

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

export abstract class Component<E = unknown> {
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


export abstract class Container<E = unknown, Child = unknown> extends Component<E> {
  static default() { return new NullContainer(null); }
  static coerce(e: any): Container<unknown> { return e instanceof Container ? e : this.default(); }

  children: Children<Child>;

  constructor(root: Container<E, Child> | null) {
    super();
    if(this instanceof NullContainer) {
      this.children = new NullChildren(this);
    } else {
      this.children = root?.children ?? new Children(this);
    }
  }

  provide(): { [key: string]: any } {
    return {
      parent: new ComponentProperty().set(this).freeze(),
    };
  }
}

export class NullContainer<T> extends Container<any, T> {
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
  childCount = PropertyFactory.int();

  constructor(parent: any) {
    this.parent = parent as Container<Object>;
  }

  append(child: Component<T>) {
    this.childCount.set(this.childCount.get() + 1)
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

class NullChildren<T> extends Children<T> {
  append(_child: Component<T>) {}
  update(_child: Component<T>) {}
}

export class Text extends Component<Text> {
  #t: 'Text' = 'Text';
  private model = new Model({
    content: PropertyFactory.string(() => this._contentChanged()),
  });

  get bindings() {
    return this.model.bindings;
  }

  get props() {
    return this.model.props;
  }

  getRoots() { return [this]; }

  protected _contentChanged() {}
}

export type FrameSize = {
  width: Property<Length>,
  height: Property<Length>,
}

export class Rect extends Container<Rect> {
  #t: 'Rect' = 'Rect';
  private model = new Model({
    fill: PropertyFactory.brush(() => this._fillChanged()),
    x1: PropertyFactory.length(() => this._x1Changed()),
    y1: PropertyFactory.length(() => this._y1Changed()),
    x2: PropertyFactory.length(() => this._x2Changed()),
    y2: PropertyFactory.length(() => this._y2Changed()),
    width: PropertyFactory.length(() => this._widthChanged()),
    height: PropertyFactory.length(() => this._heightChanged()),
  });
  protected privateModel = new Model({
    parentSize: {
      width: PropertyFactory.length(() => this._parentWidthChanged()),
      height: PropertyFactory.length(() => this._parentHeightChanged()),
    }
  });

  scaleToParent = new PropertyPreset(PropertyFactory.float())
    .addChild(
      this.bindings.x1,
      [this.privateModel.bindings.parentSize.width] as const,
      ([value, width]) => width.mul(value/2).neg(),
    )
    .addChild(
      this.bindings.x2,
      [this.privateModel.bindings.parentSize.width] as const,
      ([value, width]) => width.mul(value/2),
    )
    .addChild(
      this.bindings.y1,
      [this.privateModel.bindings.parentSize.height] as const,
      ([value, height]) => height.mul(value/2).neg(),
    )
    .addChild(
      this.bindings.y2,
      [this.privateModel.bindings.parentSize.height] as const,
      ([value, height]) => height.mul(value/2),
    );;

  readonly events = {
    pointer: pointerEvents(this as Rect),
  };

  constructor() {
    super(null);
    this.bindings.width.bind(
      [this.bindings.x1, this.bindings.x2] as const,
      ([x1, x2]) => x2.sub(x1)
    ).freeze();
    this.bindings.height.bind(
      [this.bindings.y1, this.bindings.y2] as const,
      ([y1, y2]) => y2.sub(y1)
    ).freeze();
  }

  get bindings() {
    return this.model.bindings;
  }

  get props() {
    return this.model.props;
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
      this.privateModel.bindings.parentSize.width.bind([deps.frameSize.width as Property<Length>]);
      this.privateModel.bindings.parentSize.height.bind([deps.frameSize.height as Property<Length>]);
    }
  }

  getRoots() { return [this]; }

  protected _fillChanged() {}
  protected _x1Changed() {}
  protected _y1Changed() {}
  protected _x2Changed() {}
  protected _y2Changed() {}
  protected _widthChanged() {}
  protected _heightChanged() {}
  protected _parentWidthChanged() {}
  protected _parentHeightChanged() {}
}

const COMPONENTS: Component[] = [null as never];

export class ComponentProperty {
  property: Property<number>;
  lastValue = 0;

  constructor(notify?: () => void) {
    this.property = PropertyFactory.int(notify && (() => {
      if(this.lastValue != 0) {
        notify()
      }
    }));
    this.set(new NullComponent());
  }

  get() { return COMPONENTS[this.property.get()]; }
  set(component: Component) {
    const value = COMPONENTS.length;
    COMPONENTS.push(component);
    this.property.set(value);
    this.changed();
    return this;
  }
  freeze() {
    this.property.freeze();
    return this;
  }
  bind<P extends readonly Property<any>[]>(parents: P, fn: Transformer<P, Component>) {
    this.property.bind(parents, v => {
      const component = fn(v);
      const value = COMPONENTS.length;
      COMPONENTS.push(component);
      this.changed();
      return value;
    });
    return this;
  }
  unbind() {
    this.property.unbind();
   return this;
  }
  toString() { return this.property.toString(); }

  private changed() {
    const value = this.property.get();
    if(this.lastValue !== 0) {
      delete COMPONENTS[this.lastValue];
    }
    this.lastValue = value;
  }
}

export class Slot<T = unknown> extends Component<T> {
  #t: 'Slot' = 'Slot';
  private model = new Model({
    component: new ComponentProperty(() => this._updateComponent()) as unknown as Property<Component<T>>,
    insert: PropertyFactory.boolean(() => this._updateInsert()),
  });

  prev: Component<T>;
  parent = new ComponentProperty() as unknown as Property<Container<unknown, T>>;
  injected: { [key: string]: any; } = {};
  parentHasBeenSet = false;
  emptyComponent: Component<T>;

  constructor() {
    super();
    this.parent.set(new NullContainer<T>(null));
    this.emptyComponent = this.props.component;
    this.prev = this.props.component;
  }

  inject(deps: { [key: string]: any; }) {
    this.parent.set(deps.parent.get());
    this.parentHasBeenSet = true;
    this.injected = deps;
    this._updateComponent();
  }

  get bindings() {
    return this.model.bindings;
  }

  get props() {
    return this.model.props;
  }

  getRoots(): T[] {
    const q = this.props.component.getRoots();
    const r = this.emptyComponent.getRoots();
    return this.props.insert ? q : r;
  }

  private _updateComponent(/*prev: Component, cur: Component*/) {
    const cur = this.props.component;
    cur.inject(this.injected);
    if(this.parentHasBeenSet && this.prev !== cur) {
      this.parent.get().children.update(this);
    }
    this.prev = cur;
  }

  private _updateInsert() {
    if(this.parentHasBeenSet) {
      this.parent.get().children.update(this);
    }
  }
}

export class Pane extends Container<Pane> {
  #t: 'Pane' = 'Pane';
  private privateModel = new Model({
    siblingCount: PropertyFactory.int(),
    padding: PropertyFactory.length(),
    selfSize: {
      width: PropertyFactory.length(),
      height: PropertyFactory.length(),
    },
  });
  constructor() {
    super(null);
  }

  getRoots() { return [this]; }

  inject(deps: { [key: string]: any; }): void {
    if(deps.layoutInfo) {
      this.privateModel.bindings.siblingCount.bind([deps.layoutInfo.itemCount as Property<number>]);
      this.privateModel.bindings.padding.bind([deps.layoutInfo.padding as Property<Length>]);
    }
    if(deps.frameSize) {
      this.privateModel.bindings.selfSize.width.bind(
        [
          deps.frameSize.width as Property<Length>,
          this.privateModel.bindings.padding,
          this.privateModel.bindings.siblingCount,
        ] as const,
        ([width, padding, siblingCount]) => {
          return width.sub(padding.mul(2)).div(siblingCount);
        },
      );
      this.privateModel.bindings.selfSize.height.bind(
        [
          deps.frameSize.height as Property<Length>,
          this.privateModel.bindings.padding,
          this.privateModel.bindings.siblingCount,
        ] as const,
        ([height, padding, siblingCount]) => height.sub(padding.mul(2)),
      );
    }
  }
  provide() {
    return {
      ...super.provide(),
      frameSize: this.privateModel.bindings.selfSize,
      layoutInfo: {
        itemCount: this.children.childCount,
      },
    };
  }
}

export class Layout extends Container<Layout, Pane> {
  #t: 'Layout' = 'Layout';
  private publicModel = new Model({
    padding: PropertyFactory.length(() => this._paddingChanged()),
    layout: PropertyFactory.layout(() => this._layoutChanged()),
  });
  protected privateModel = new Model({
    parentSize: {
      width: PropertyFactory.length(),
      height: PropertyFactory.length(),
    },
  });

  constructor() {
    super(null);
  }

  get bindings() {
    return this.publicModel.bindings;
  }

  get props() {
    return this.publicModel.props;
  }

  inject(deps: { [key: string]: any }) {
    if(deps.frameSize) {
      this.privateModel.bindings.parentSize.width.bind([deps.frameSize.width as Property<Length>]);
      this.privateModel.bindings.parentSize.height.bind([deps.frameSize.height as Property<Length>]);
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

  protected _layoutChanged() {}
  protected _paddingChanged() {}
}

export class Repeater<V extends Value = Value, E = unknown> extends Component<E> {
  #t : 'Repeater' = 'Repeater';
  private model = new Model({
    collection: PropertyFactory.iter(() => this._updateItems()),
  });
  components: Component<E>[] = [];
  parent = new ComponentProperty() as unknown as Property<Container>;

  constructor(private proc: (i: number, p: V) => Component<E>[]) {
    super();
    this.parent.set(new NullContainer(null));
  }

  get bindings() {
    return this.model.bindings;
  }

  get props() {
    return this.model.props;
  }

  inject(deps: { [key: string]: any; }): void {
    this.parent.set(deps.parent.get());
    for(const item of this.components) {
      item.inject(deps);
    }
  }

  getRoots() {
    return this.components.map(e => e.getRoots()).flat();
  }

  _updateItems() {
    const all: Component<E>[][] = [];
    let i = 0;
    for(const p of this.props.collection) {
      const component = this.proc.call(null, i++, p as V);
      all.push(component);
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
  Repeater<V extends Value, E = unknown>(
    proc: (i: number, p: V) => Component<E>[],
  ): Repeater<V, E>;
}
