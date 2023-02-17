import { Length } from './length';
import { Brush } from './brush';
import { Children, Component, Element } from './common';
import { Binding } from './binding';

export * from './common';
export * from './length';
export * from './brush';
export * from './transition';
export * from './binding';

export type EventHandler<E extends Element> = (el: E, e: Event) => unknown;

export interface EventEmitterAdapter<E extends Element> {
  addListener(handler: EventHandler<E>, name: string): void;
  removeListener(name: string): void;
}

export class EventEmitter<E extends Element> {
  listeners: { [key: string]: EventHandler<E> } = {};
  adapter?: EventEmitterAdapter<E>;

  constructor(public element: E) {}

  addListener(handler: EventHandler<E>, name: string) {
    this.listeners[name] = handler;
    this.adapter?.addListener(handler, name);
  }

  removeListener(name: string) {
    delete this.listeners[name];
    this.adapter?.removeListener(name);
  }
}

function pointerEvents<E extends Element>(element: E) {
  return {
    click: new EventEmitter<E>(element),
  }
}

export interface Container<E extends Element = Element> extends Component<E> {
  children: Children<E>;
  getRoot(): E;
}

export abstract class ContainerElement extends Element implements Container<Element> {
  children: Children<Element>;

  constructor() {
    super();
    this.children = new Children(this.getRoot());
  }

  getRoot() {
    return this;
  }
}

export type FrameSize = {
  width: Binding<Length>,
  height: Binding<Length>,
}

export class Rect extends ContainerElement {
  readonly fill = new Binding(Brush);
  readonly x1 = new Binding(Length);
  readonly y1 = new Binding(Length);
  readonly x2 = new Binding(Length);
  readonly y2 = new Binding(Length);
  readonly frameSize = {
    width: new Binding(Length),
    height: new Binding(Length),
  };
  readonly selfSize = {
    width: new Binding(Length).connect([this.x1, this.x2], ([x1, x2]) => x2.sub(x1)),
    height: new Binding(Length).connect([this.y1, this.y2], ([y1, y2]) => y2.sub(y1)),
  };
  readonly events = {
    pointer: pointerEvents(this),
  };

  constructor() {
    super();
  }

  inject(deps: { [key: string]: any }) {
    if(deps.frameSize) {
      this.frameSize.width.connect([deps.frameSize.width]);
      this.frameSize.height.connect([deps.frameSize.height]);
    }
  }

  provide() {
    return { frameSize: this.selfSize };
  }
}

export interface Dom {
  Rect(): Rect;
}
