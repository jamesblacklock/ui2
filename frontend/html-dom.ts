import * as dom from './dom';
import { Color } from './brush';

export * from './dom';

class Vw extends dom.Length {
  constructor(public value: number) {
    super();
  }
  equals(other: dom.Property): boolean {
    return other instanceof Vw && this.value === other.value;
  }
  add(other: dom.Length): dom.Length {
    if(other instanceof Vw) {
      return new Vw(this.value + other.value);
    }
    return super.add(other)
  }
  sub(other: dom.Length): dom.Length {
    if(other instanceof Vw) {
      return new Vw(this.value - other.value);
    }
    return super.sub(other)
  }
  mul(fac: number): dom.Length {
    return new Vw(this.value * fac);
  }
  div(fac: number): dom.Length {
    return new Vw(this.value / fac);
  }
  neg(): dom.Length {
    return new Vw(-this.value);
  }
}

class Vh extends dom.Length {
  constructor(public value: number) {
    super();
  }
  equals(other: dom.Property): boolean {
    return other instanceof Vh && this.value === other.value;
  }
  add(other: dom.Length): dom.Length {
    if(other instanceof Vh) {
      return new Vh(this.value + other.value);
    }
    return super.add(other);
  }
  sub(other: dom.Length): dom.Length {
    if(other instanceof Vh) {
      return new Vh(this.value - other.value);
    }
    return super.sub(other);
  }
  mul(fac: number): dom.Length {
    return new Vh(this.value * fac);
  }
  div(fac: number): dom.Length {
    return new Vh(this.value / fac);
  }
  neg(): dom.Length {
    return new Vh(-this.value);
  }
}

function lengthToCss(length: dom.Length, top = true): string {
  let css;
  if(length instanceof dom.Px) {
    return `${length.value}px`;
  } else if(length instanceof Vh) {
    return `${length.value * 100}vh`;
  } else if(length instanceof Vw) {
    return `${length.value * 100}vw`;
  } else if(length instanceof dom.LengthAddition) {
    css = `(${lengthToCss(length.op1, false)} + ${lengthToCss(length.op2, false)})`;
  } else if(length instanceof dom.LengthMultiplication) {
    css = `(${lengthToCss(length.op1, false)} * ${length.op2})`;
  } else if(length instanceof dom.LengthSubtraction) {
    css = `(${lengthToCss(length.op1, false)} - ${lengthToCss(length.op2, false)})`;
  } else if(length instanceof dom.LengthDivision) {
    css = `(${lengthToCss(length.op1, false)} / ${length.op2})`;
  } else if(length instanceof dom.LengthNegation) {
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

interface HtmlComponent {
  html: Node;
}

class ChildrenAdapter implements dom.ChildrenAdapter {
  constructor(public html: Node) {}

  ensureHtml(element: HtmlComponent) {
    if(element.html === undefined) {
      let node = document.createElement('div');
      node.style.display = 'none';
      element.html = node;
    }
  }
  append(element: HtmlComponent) {
    this.ensureHtml(element);
    this.html.appendChild(element.html);
  }
  replace(cur: HtmlComponent, next: HtmlComponent) {
    this.ensureHtml(next);
    this.html.replaceChild(next.html, cur.html);
  }
}

class EventEmitterAdapter<E> implements dom.EventEmitterAdapter<E> {
  listeners: { [key: string]: (e: Event) => unknown } = {};

  constructor(public name: string, public element: E) {}

  addListener(handler: dom.EventHandler<E>, name: string) {
    const bound = (event: Event) => handler(this.element, convertEvent(event));
    this.listeners[name] = bound;
    (this.element as HtmlComponent).html.addEventListener(this.name, bound);
  }

  removeListener(name: string) {
    const handler = this.listeners[name];
    delete this.listeners[name];
    (this.element as HtmlComponent).html.removeEventListener(this.name, handler);
  }
}

export class Rect extends dom.Rect implements HtmlComponent {
  html: HTMLDivElement;
  transitionUpdateRequest?: number;
  fillUpdateRequest?: number;
  positionUpdateRequest?: number;
  transitions: { [key: string]: string } = {};

  constructor(dom: Dom) {
    super();
    this.bindings.fill.onChange(() => this._fillChanged());
    this.bindings.x1.onChange(() => this._positionChanged());
    this.bindings.y1.onChange(() => this._positionChanged());
    this.bindings.x2.onChange(() => this._positionChanged());
    this.bindings.y2.onChange(() => this._positionChanged());
    this.privateModel.bindings.parentSize.width.onChange(() => this._positionChanged());
    this.privateModel.bindings.parentSize.height.onChange(() => this._positionChanged());

    this.html = dom.document.createElement('div');
    this.html.style.position = 'absolute';
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
    this._updateTransition('background', this.bindings.fill.transition);
    cancelAnimationFrame(this.fillUpdateRequest ?? 0);

    this.fillUpdateRequest = requestAnimationFrame(() => {
      if(this.html === undefined) {
        return;
      }
      const fill = this.props.fill;
      if(fill instanceof Color) {
        this.html.style.background = colorToCss(fill);
      }
    });
  }
  _positionChanged() {
    if(this.html === undefined) {
      return;
    }
    this._updateTransition('left', this.bindings.x1.transition);
    this._updateTransition('right', this.bindings.x2.transition);
    this._updateTransition('top', this.bindings.y1.transition);
    this._updateTransition('bottom', this.bindings.y2.transition);

    cancelAnimationFrame(this.positionUpdateRequest ?? 0);
    this.positionUpdateRequest = requestAnimationFrame(() => {

      const parentW = this.privateModel.props.parentSize.width;
      const parentH = this.privateModel.props.parentSize.height;

      const left = this.props.x1.add(parentW.mul(0.5));
      const top = this.props.y1.add(parentH.mul(0.5));
      const right = parentW.mul(0.5).sub(this.props.x2);
      const bottom = parentH.mul(0.5).sub(this.props.y2);

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

export class Text extends dom.Text implements HtmlComponent {
  html: HTMLElement;
  constructor(dom: Dom) {
    super();
    this.html = dom.document.createElement('span');
    this.bindings.content.onChange(this.updateText.bind(this));
  }
  updateText(_prev: dom.String, cur: dom.String) {
    this.html.textContent = cur.value;
  }
}

export class Pane extends dom.Pane implements HtmlComponent {
  html: HTMLElement;
  constructor(dom: Dom) {
    super();
    this.html = dom.document.createElement('div');
    this.html.style.flexGrow = '1';
    this.html.style.position = 'relative';

    this.children.adapter = new ChildrenAdapter(this.html);
  }
}

export class Layout extends dom.Layout implements HtmlComponent {
  html: HTMLElement;
  sizeUpdateRequest?: number;
  constructor(dom: Dom) {
    super();
    this.html = dom.document.createElement('div');
    this.children.adapter = new ChildrenAdapter(this.html);
    this.html.style.position = 'absolute';
    this.html.style.display = 'flex';
    this.html.style.flexDirection = 'row';
    this.html.style.left = '0px';
    this.html.style.top = '0px';
    this.html.style.right = '0px';
    this.html.style.bottom = '0px';

    this.bindings.layout.onChange(() => this._layoutChanged());
    this.bindings.padding.onChange(() => this._paddingChanged());
    this._layoutChanged();
  }
  _layoutChanged() {
    this.html.style.flexDirection = this.props.layout === dom.Enum.Layout.Row ? 'row' : 'column';
  }
  _paddingChanged() {
    this.html.style.padding = lengthToCss(this.props.padding);
  }
}

export class Body extends dom.Container<Body> implements HtmlComponent {
  html: HTMLElement;
  constructor(dom: Dom) {
    super(null);
    this.html = dom.document.body;
    this.html.style.position = 'absolute';
    this.html.style.width = '100vw';
    this.html.style.height = '100vh';
    this.html.style.padding = '0';
    this.html.style.margin = '0';
    this.children.adapter = new ChildrenAdapter(this.html);
  }

  provide(): { [key: string]: any } {
    return {
      parent: new dom.Binding(dom.Container).set(this),
      frameSize: {
        width: new dom.Binding(dom.Length).set(new Vw(1)),
        height: new dom.Binding(dom.Length).set(new Vh(1)),
      },
    }
  }

  getRoot() {
    return this;
  }
}

export class Dom implements dom.Dom {
  public document: Document;
  public body: Body;

  constructor(htmlDocument: Document) {
    this.document = htmlDocument;
    this.body = new Body(this);
  }
  Slot<T>(): dom.Slot<T> {
    return new dom.Slot();
  }
  Rect() {
    return new Rect(this);
  }
  Text() {
    return new Text(this);
  }
  Layout() {
    return new Layout(this);
  }
  Pane() {
    return new Pane(this);
  }
  Empty() {
    return new dom.Empty();
  }
}
