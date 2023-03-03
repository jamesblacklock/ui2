import * as dom from './dom';

export * from './dom';

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
  append(elements: HtmlComponent[]) {
    for(const element of elements) {
      this.ensureHtml(element);
      this.html.appendChild(element.html);
    }
  }
  replace(cur: HtmlComponent[], next: HtmlComponent[]) {
    for(const element of next) {
      this.ensureHtml(element);
    }
    for(const element of next) {
      this.html.insertBefore(element.html, cur[0].html);
    }
    for(const element of cur) {
      this.html.removeChild(element.html);
    }
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
    if(this.html === undefined || this.fillUpdateRequest !== undefined) {
      return;
    }
    cancelAnimationFrame(this.fillUpdateRequest ?? 0);

    this.fillUpdateRequest = window.requestAnimationFrame(() => {
      this.fillUpdateRequest = undefined;
      if(this.html === undefined) {
        return;
      }
      const fill = this.props.fill;
      this.html.style.background = fill.toCss();
    });
  }
  _x1Changed() { this._positionChanged(); }
  _y1Changed() { this._positionChanged(); }
  _x2Changed() { this._positionChanged(); }
  _y2Changed() { this._positionChanged(); }
  _parentWidthChanged() { this._positionChanged(); }
  _parentHeightChanged() { this._positionChanged(); }
  _positionChanged() {
    if(this.html === undefined || this.positionUpdateRequest != undefined) {
      return;
    }

    this.positionUpdateRequest = window.requestAnimationFrame(() => {
      this.positionUpdateRequest = undefined;
      const parentW = this.privateModel.props.parentSize.width;
      const parentH = this.privateModel.props.parentSize.height;

      const left = this.props.x1.add(parentW.mul(0.5));
      const top = this.props.y1.add(parentH.mul(0.5));
      const right = parentW.mul(0.5).sub(this.props.x2);
      const bottom = parentH.mul(0.5).sub(this.props.y2);

      this.html.style.left = left.toCss();
      this.html.style.top = top.toCss();
      this.html.style.right = right.toCss();
      this.html.style.bottom = bottom.toCss();
    });
  }
  _updateTransition(key: string, transition?: dom.Transition) {
    if(transition instanceof dom.Ease) {
      this.transitions[key] = `ease ${transition.time}ms`;
    } else {
      delete this.transitions[key];
    }
    if(this.transitionUpdateRequest === undefined) {
      this.transitionUpdateRequest = window.requestAnimationFrame(() => {
          this.transitionUpdateRequest = undefined
          this.html.style.transition = Object.entries(this.transitions).map(([k, v]) => `${k} ${v}`).join(', ');
      });
    }
  }
}

export class Text extends dom.Text implements HtmlComponent {
  html: HTMLElement;
  constructor(dom: Dom) {
    super();
    this.html = dom.document.createElement('span');
  }
  _contentChanged() {
    this.html.textContent = this.props.content;
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
    this._layoutChanged();
  }
  _layoutChanged() {
    this.html.style.flexDirection = this.props.layout === dom.Enum.Layout.Row ? 'row' : 'column';
  }
  _paddingChanged() {
    this.html.style.padding = this.props.padding.toCss();
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
      parent: new dom.ComponentProperty().set(this),
      frameSize: {
        width: dom.PropertyFactory.length().set(dom.Length.__htmlVw(1)),
        height: dom.PropertyFactory.length().set(dom.Length.__htmlVh(1)),
      },
    }
  }

  getRoots() { return [this]; }
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
  Repeater<V extends dom.Value, E = unknown>(
    proc: (i: number, p: V) => dom.Component<E>[]
  ) {
    return new dom.Repeater(proc);
  }
}
