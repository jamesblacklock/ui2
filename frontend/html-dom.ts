import {
  Length,
  LengthAddition,
  LengthDivision,
  LengthMultiplication,
  LengthNegation,
  LengthSubtraction,
  Px,
} from './length';
import * as dom from './dom';
import { Color } from './brush';

export * from './dom';

class Vw extends Length {
  constructor(public value: number) {
    super();
  }
  add(other: Length): Length {
    if(other instanceof Vw) {
      return new Vw(this.value + other.value);
    }
    return super.add(other)
  }
  sub(other: Length): Length {
    if(other instanceof Vw) {
      return new Vw(this.value - other.value);
    }
    return super.sub(other)
  }
  mul(fac: number): Length {
    return new Vw(this.value * fac);
  }
  div(fac: number): Length {
    return new Vw(this.value / fac);
  }
  neg(): Length {
    return new Vw(-this.value);
  }
}

class Vh extends Length {
  constructor(public value: number) {
    super();
  }
  add(other: Length): Length {
    if(other instanceof Vh) {
      return new Vh(this.value + other.value);
    }
    return super.add(other);
  }
  sub(other: Length): Length {
    if(other instanceof Vh) {
      return new Vh(this.value - other.value);
    }
    return super.sub(other);
  }
  mul(fac: number): Length {
    return new Vh(this.value * fac);
  }
  div(fac: number): Length {
    return new Vh(this.value / fac);
  }
  neg(): Length {
    return new Vh(-this.value);
  }
}

function lengthToCss(length: Length, top = true): string {
  let css;
  if(length instanceof Px) {
    return `${length.value}px`;
  } else if(length instanceof Vh) {
    return `${length.value * 100}vh`;
  } else if(length instanceof Vw) {
    return `${length.value * 100}vw`;
  } else if(length instanceof LengthAddition) {
    css = `(${lengthToCss(length.op1, false)} + ${lengthToCss(length.op2, false)})`;
  } else if(length instanceof LengthMultiplication) {
    css = `(${lengthToCss(length.op1, false)} * ${length.op2})`;
  } else if(length instanceof LengthSubtraction) {
    css = `(${lengthToCss(length.op1, false)} - ${lengthToCss(length.op2, false)})`;
  } else if(length instanceof LengthDivision) {
    css = `(${lengthToCss(length.op1, false)} / ${length.op2})`;
  } else if(length instanceof LengthNegation) {
    css = `(0px - ${lengthToCss(length.op1, false)})`;
  } else {
    throw new Error(`length type not recognized: ${length.constructor.name}`);
  }

  if(top) {
    return `calc${css}`;
  }

  return css;
}

function colorToCss(color: Color) {
  const r = color.r * 255;
  const g = color.g * 255;
  const b = color.b * 255;
  const a = color.a * 255;
  return `rgba(${r}, ${g}, ${b}, ${a})`;
}

function convertEvent(event: Event) {
  return event;
}

interface Element extends dom.Element {
  html: Node;
}

class ChildrenAdapter implements dom.ChildrenAdapter<Element> {
  constructor(public html: Node) {}

  append(element: Element) {
    this.html.appendChild(element.html);
  }
}

class EventEmitterAdapter<E extends Element> implements dom.EventEmitterAdapter<E> {
  listeners: { [key: string]: (e: Event) => unknown } = {};

  constructor(public name: string, public element: E) {}

  addListener(handler: dom.EventHandler<E>, name: string) {
    const bound = (event: Event) => handler(this.element, convertEvent(event));
    this.listeners[name] = bound;
    this.element.html.addEventListener(this.name, bound);
  }

  removeListener(name: string) {
    const handler = this.listeners[name];
    delete this.listeners[name];
    this.element.html.removeEventListener(this.name, handler);
  }
}

export class Rect extends dom.Rect implements Element {
  html: HTMLDivElement;
  transitionUpdateRequest?: number;
  fillUpdateRequest?: number;
  positionUpdateRequest?: number;
  transitions: { [key: string]: string } = {};

  constructor(document: Document) {
    super();
    this.fill.onChange(() => this._fillChanged());
    this.x1.onChange(() => this._positionChanged());
    this.y1.onChange(() => this._positionChanged());
    this.x2.onChange(() => this._positionChanged());
    this.y2.onChange(() => this._positionChanged());
    this.frameSize.width.onChange(() => this._positionChanged());
    this.frameSize.height.onChange(() => this._positionChanged());

    this.html = document.createElement('div');
    this.html.ontransitionend = e => {
      e.stopPropagation();
      this.html.style.transition = '';
    };
    this.children.adapter = new ChildrenAdapter(this.html);
    this.events.pointer.click.adapter = new EventEmitterAdapter('click', this);
    this._fillChanged();
    this._positionChanged();
  }

  _fillChanged() {
    if(this.html === undefined) {
      return;
    }
    this._updateTransition('background', this.fill.transition);
    cancelAnimationFrame(this.fillUpdateRequest ?? 0);

    this.fillUpdateRequest = requestAnimationFrame(() => {
      if(this.html === undefined) {
        return;
      }
      const fill = this.fill.get();
      if(fill instanceof Color) {
        this.html.style.background = colorToCss(fill);
      }
    });
  }
  _positionChanged() {
    if(this.html === undefined) {
      return;
    }
    this._updateTransition('left', this.x1.transition);
    this._updateTransition('right', this.x2.transition);
    this._updateTransition('top', this.y1.transition);
    this._updateTransition('bottom', this.y2.transition);

    cancelAnimationFrame(this.positionUpdateRequest ?? 0);
    this.positionUpdateRequest = requestAnimationFrame(() => {
      const parentW = this.frameSize.width.get();
      const parentH = this.frameSize.height.get();

      const left = this.x1.get().add(parentW.mul(0.5));
      const top = this.y1.get().add(parentH.mul(0.5));
      const right = parentW.mul(0.5).sub(this.x2.get());
      const bottom = parentH.mul(0.5).sub(this.y2.get());

      this.html.style.position = 'absolute';
      this.html.style.left = lengthToCss(left);
      this.html.style.top = lengthToCss(top);
      this.html.style.right = lengthToCss(right);
      this.html.style.bottom = lengthToCss(bottom);
    });
  }
  _updateTransition(key: string, transition?: dom.Transition) {
    if(transition instanceof dom.Ease) {
      this.transitions[key] = `ease ${transition.time}ms`;
    } else {
      delete this.transitions[key];
    }
    cancelAnimationFrame(this.transitionUpdateRequest ?? 0);
    this.transitionUpdateRequest = window.requestAnimationFrame(() => {
        this.html.style.transition = Object.entries(this.transitions).map(([k, v]) => `${k} ${v}`).join(', ');
    });
  }
}

export class Body extends dom.ContainerElement implements Element {
  html: HTMLElement;
  constructor(dom: Dom) {
    super();
    this.html = dom.document.body;
    this.html.style.position = 'absolute';
    this.html.style.width = '100vw';
    this.html.style.height = '100vh';
    this.html.style.padding = '0';
    this.html.style.margin = '0';
    this.children.adapter = new ChildrenAdapter(this.html);
  }

  provide() {
    return {
      frameSize: {
        width: new dom.Binding(new Vw(1)),
        height: new dom.Binding(new Vh(1)),
      },
    }
  }
}

export class Dom implements dom.Dom {
  public document: Document;
  public body: Body;

  constructor(htmlDocument: Document) {
    this.document = htmlDocument;
    this.body = new Body(this);
  }
  Rect() {
    return new Rect(this.document);
  }
}
