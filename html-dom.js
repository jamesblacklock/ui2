const dom = require('./dom');

function lengthToCss(length) {
  if(length instanceof dom.Px) {
    return `${length.px}px`;
  } else if(length instanceof dom.Frac) {
    return `${length.frac * 100}%`;
  } else {
    return '';
  }
}

function colorToCss(color) {
  const r = color.r * 255;
  const g = color.g * 255;
  const b = color.b * 255;
  const a = color.a * 255;
  return `rgba(${r}, ${g}, ${b}, ${a})`;
}

function convertEvent(event) {
  return event;
}

class ChildrenAdapter {
  constructor(html) {
    this.html = html;
  }

  append(element) {
    this.html.appendChild(element.element.html);
  }
}

class Event {
  constructor(name, element) {
    this.listeners = {};
    this.element = element;
    this.name = name;
  }

  addListener(handler, name) {
    const bound = event => handler(this.element, convertEvent(event));
    this.listeners[name] = bound;
    this.element.html.addEventListener(this.name, bound);
  }

  removeListener(name) {
    const handler = this.listeners[name]
    delete this.listeners[name];
    this.element.html.removeEventListener(this.name, handler);
  }
}

class Rect extends dom.Rect {
  constructor(...args) {
    super(...args);
    this.html = document.createElement('div');
    this.children.adapter = new ChildrenAdapter(this.html);
    this.events.click.adapter = new Event('click', this);
    this.transitionUpdateTriggered = false;
    this.transitions = {};
    this._fillChanged();
    this['_pos.leftChanged']();
    this['_pos.rightChanged']();
    this['_pos.topChanged']();
    this['_pos.bottomChanged']();
  }

  _propertyChanged(name) {
    this[`_${name}Changed`]();
  }
  _fillChanged() {
    if(this.html === undefined) {
      return;
    }
    const prevTransition = this.transitions['background'];
    if(this.fill.transition instanceof dom.Ease) {
      this.transitions['background'] = `ease ${this.fill.transition.time}ms`;
    } else {
      this.transitions['background'] = '';
    }
    if(prevTransition !== this.transitions['top']) {
      this._updateTransition();
    }
    const fill = this.fill.get();
    if(fill instanceof dom.Color) {
      this.html.style.background = colorToCss(fill);
    }
  }
  '_pos.leftChanged'() {
    this._positionChanged('left');
  }
  '_pos.topChanged'() {
    this._positionChanged('top');
  }
  '_pos.rightChanged'() {
    this._positionChanged('right', true);
  }
  '_pos.bottomChanged'() {
    this._positionChanged('bottom', true);
  }
  _positionChanged(key, invert) {
    if(this.html === undefined) {
      return;
    }
    const prevTransition = this.transitions[key];
    if(this.pos.bindings[key].transition instanceof dom.Ease) {
      this.transitions[key] = `ease ${this.pos.bindings[key].transition.time}ms`;
    } else {
      this.transitions[key] = '';
    }
    if(prevTransition !== this.transitions[key]) {
      this._updateTransition();
    }
    requestAnimationFrame(() => {
      const position = this.pos.bindings[key].get();
      const cssLength = lengthToCss(position.length);
      if(position instanceof dom.Anchor) {
        this.html.style.position = 'absolute';
        if(position.from === 'start') {
          if(!invert) {
            this.html.style[key] = cssLength;
          } else {
            this.html.style[key] = `calc(100% - ${cssLength})`;
          }
        } else if(position.from === 'end') {
          if(invert) {
            this.html.style[key] = cssLength;
          } else {
            this.html.style[key] = `calc(100% - ${cssLength})`;
          }
        } else {
          if(invert) {
            this.html.style[key] = `calc(50% - ${cssLength})`;
          } else {
            this.html.style[key] = `calc(50% + ${cssLength})`;
          }
        }
      }
    });
  }
  _updateTransition() {
    if(!this.transitionUpdateTriggered) {
      this.transitionUpdateTriggered = true;
      window.requestAnimationFrame(() => {
        this.html.style.transition = Object.entries(this.transitions).map(([k, v]) => `${k} ${v}`).join(', ');
        this.transitionUpdateTriggered = false;
      });
    }
  }
}

class Body extends dom.Container {
  constructor(dom) {
    super();
    this.html = dom.document.body;
    this.children.adapter = new ChildrenAdapter(this.html);
  }
}

class Dom extends dom.Dom {
  constructor(htmlDocument) {
    super();
    this.document = htmlDocument;
    this.body = new Body(this);
  }
  Rect(...args) {
    return new Rect(...args);
  }
}

module.exports = {
  ...dom,
  Dom,
  Rect,
}