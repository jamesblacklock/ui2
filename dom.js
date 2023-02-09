class AbstractUnimplemented extends Error {
  constructor(name) {
    super(`${name}: abstract method unimplemented`);
  }
}

class Property {
  equals(other) {
    return false;
  }
}

class Color extends Property {
  static get RED() { return Color.rgb(1.0, 0, 0) };
  static get TRANSPARENT() { return Color.rgba(0, 0, 0, 0) };

  static rgb(r, g, b) {
    return Color.rgba(r, g, b, 1);
  }

  static rgba(r, g, b, a) {
    const color = new Color();
    color.r = Math.max(Math.min(r, 1), 0);
    color.g = Math.max(Math.min(g, 1), 0);
    color.b = Math.max(Math.min(b, 1), 0);
    color.a = Math.max(Math.min(a, 1), 0);
    return color;
  }

  constructor() {
    super();
  }
}

class Point extends Property {
  constructor(x, y) {
    super();
    this.x = x;
    this.y = y;
  }
}

class Anchor extends Property {
  constructor(length = undefined, from = 'center') {
    super();
    this.length = length ?? new Px(0);
    this.from = from;
  }
  static fromCenter(length) {
    return new Anchor(length);
  }
  static fromStart(length) {
    return new Anchor(length, 'start');
  }
  static fromEnd(length) {
    return new Anchor(length, 'end');
  }
}

class Px extends Property {
  constructor(px) {
    super();
    this.px = px;
  }
  equals(other) {
    return other instanceof Px && other.px === this.px;
  }
}

class Frac extends Property {
  constructor(frac) {
    super();
    this.frac = frac;
  }
}

class Children {
  constructor() {
    this.arr = [];
  }

  append(element) {
    this.arr.push(element);
    this.adapter?.append(element);
  }

  get(index) {
    return this.arr[index];
  }

  get length() {
    return this.arr.length;
  }
}

class Event {
  constructor(element) {
    this.listeners = {};
    this.element = element;
  }

  addListener(handler, name) {
    this.listeners[name] = handler;
    this.adapter?.addListener(handler, name);
  }

  removeListener(name) {
    delete this.listeners[name];
    this.adapter?.removeListener(name);
  }
}

const PointerEvents = {
  __init__(self) {
    self.events.click = new Event(self);
  },
};

function Extension(baseClass, extensions) {
  const extendedClass = class extends baseClass {
    constructor(...args) {
      super(...args);
      for(const extension of extensions) {
        extension.__init__?.(this);
      }
    }
  };

  for(const extension of extensions) {
    const { __init__, ...items } = extension;
    Object.assign(extendedClass.prototype, items);
  }

  return extendedClass;
}

class Extensible {
  static extends(...extensions) {
    return Extension(this, extensions);
  }
}

class Element extends Extensible {
  constructor() {
    super();
    this.events = {};
    this.state 
  }

  get element() { return this; }
}

class Container extends Element {
  constructor() {
    super();
    this.children = new Children();
  }
}

class Ease {
  constructor(time) {
    this.time = time;
  }
}

class Binding {
  static from(ancestor, transform, transition) {
    const binding = new Binding(ancestor.property, transition, transform);
    ancestor.descendants.add(binding);
    return binding;
  }

  constructor(property, transition, transform = e => e) {
    this.property = Object.freeze(transform(property));
    this.transition = Object.freeze(transition);
    this.elements = new Map();
    this.descendants = new Set();
    this.transform = transform;
  }

  bind(element, property) {
    const bindings = this.elements.get(element) ?? new Set();
    bindings.add(property);
    this.elements.set(element, bindings);
    element._propertyChanged(property);
    return this;
  }

  unbind(element, property) {
    const bindings = this.elements.get(element) ?? new Set();
    bindings.delete(property);
    if(bindings.size === 0) {
      this.elements.delete(element);
    }
    return this;
  }

  get() {
    return this.property;
  }

  set(property) {
    this.property = this.transform(property);
    for(const [element, properties] of this.elements) {
      for(const property of properties) {
        element._propertyChanged(property);
      }
    }
    for(const binding of this.descendants) {
      binding.set(this.property);
    }
  }
}

class BindingGroup {
  constructor(element, name, bindings) {
    this.element = element;
    this.name = name;
    this.bindings = bindings;
    this.set(bindings);
  }

  set(bindings) {
    const changes = this._setBindings(this.name, this.bindings, bindings);
    this.bindings = bindings;
    for(const [binding, name] of changes) {
      binding.bind(this.element, name);
    }
  }

  get() {
    return this.bindings;
  }

  _setBindings(parentName, current, next) {
    const changes = [];
    for(const [name, value] of Object.entries(current)) {
      if(value.constructor === Object) {
        const subChanges = this._setBindings(`${parentName}.${name}`, value, next[name]);
        changes.push(...subChanges);
      }
      if(next[name] && value !== next[name]) {
        value.unbind(this.element, `${parentName}.${name}`);
      }
      if(next[name]) {
        changes.push([next[name], `${parentName}.${name}`])
      } else {
        next[name] = value;
      }
    }
    return changes;
  }
}

class Rect extends Container.extends(PointerEvents) {
  #fill = new Binding(Color.TRANSPARENT).bind(this, 'fill');
  #pos = new BindingGroup(this, 'pos', {
    left: new Binding(Anchor.fromStart(new Px(0))),
    right: new Binding(Anchor.fromStart(new Px(0))),
    top: new Binding(Anchor.fromStart(new Px(0))),
    bottom: new Binding(Anchor.fromStart(new Px(0))),
  });

  constructor({ fill, pos }) {
    super();
    if(fill) {
      this.fill = fill;
    }
    if(pos) {
      this.pos = pos;
    }
  }

  get fill() { return this.#fill; }
  get pos() { return this.#pos; }

  set fill(fill) {
    this.#fill.unbind(this, 'fill');
    this.#fill = fill.bind(this, 'fill');
  }
  set pos(pos) {
    this.#pos.set(pos);
  }

  _propertyChanged() {
    throw new AbstractUnimplemented('_propertyChanged');
  }
}

class Dom {
  Rect(...args) {
    return new Rect(...args);
  }
}

module.exports = {
  Dom,
  Rect,
  Container,
  Children,
  Color,
  Anchor,
  Px,
  Binding,
  Ease,
  Element,
  Frac,
  PointerEvents,
  Property,
};
