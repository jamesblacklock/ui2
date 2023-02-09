const { Color, Element, Px, Anchor, Binding, Ease, PointerEvents } = require('../dom');

/* header.ui

Rect {
  left: .left; // AnchorH
  right: .right; // 0px .right // Anchor.right(Length.px(0))
  top: .top;
  bottom: .top 1in;

  #logo Img {
    left: .left;
    
  }
}

*/


/*

left: Length;
right: Length;
top: Length;
bottom: Length;
color: Brush = .red;
click: Callback;

rect {
  pos: {
    left: (left);
    right: js { self => Anchor.fromLeft(new Px(self.left.get().length.px + 50)) } // (right);
    top: (top);
    bottom: (bottom);
  }
  fill: (color);
  events.click: {
    js {
      rect => {
        rect.fill.set(Color.rgb(
          Math.random(),
          Math.random(),
          Math.random(),
        ));
      }
    }
    (click)
  }
}

*/

class Square extends Element.extends(PointerEvents) {
  constructor(dom, { left, right, top, bottom }) {
    super();
    this.rect = dom.Rect({
      pos: { left, right, top, bottom },
      fill: new Binding(Color.RED, new Ease(1000)),
    });
    this.events.click = this.rect.events.click;

    this.rect.events.click.addListener(rect => {
      rect.fill.set(Color.rgb(
        Math.random(),
        Math.random(),
        Math.random(),
      ));
    }, 'color');
  }

  get element() { return this.rect; }

  get left() { return this.rect.pos.get().left; }
  get right() { return this.rect.pos.get().right; }
  get top() { return this.rect.pos.get().top; }
  get bottom() { return this.rect.pos.get().bottom; }

  set left(left) {
    this.rect.pos.set({left});
  }
  set right(right) {
    this.rect.pos.set({right});
  }
  set top(top) {
    this.rect.pos.set({top});
  }
  set bottom(bottom) {
    this.rect.pos.set({bottom});
  }
}

module.exports = function(dom, frame) {
  const positions = [
    {
      left: Anchor.fromCenter(new Px(-50)),
      right: Anchor.fromCenter(new Px(50)),
      top: Anchor.fromCenter(new Px(-50)),
      bottom: Anchor.fromCenter(new Px(50)),
    },
    {
      left: Anchor.fromStart(new Px(30)),
      right: Anchor.fromStart(new Px(30+500)),
      top: Anchor.fromEnd(new Px(40+60)),
      bottom: Anchor.fromEnd(new Px(40)),
    }
  ];
  const ease = new Ease(2000);
  window.model = {
    headerHeight: new Binding(Anchor.fromStart(new Px(80))),
    rect1: {
      left: new Binding(positions[0].left, ease),
      right: new Binding(positions[0].right, ease),
      top: new Binding(positions[0].top, ease),
      bottom: new Binding(positions[0].bottom, ease),
    },
    rect2: {
      left: new Binding(positions[1].left, ease),
      right: new Binding(positions[1].right, ease),
      top: new Binding(positions[1].top, ease),
      bottom: new Binding(positions[1].bottom, ease),
    },
  };
  const rect1 = new Square(dom, model.rect1);
  const rect2 = new Square(dom, model.rect2);
  const rect3 = new Square(dom, {
    left: model.rect1.right,
    right: Binding.from(model.rect1.right, a => new Anchor(new Px(a.length.px + 50), a.from), ease),
    top: model.rect1.top,
    bottom: model.rect1.bottom,
  });

  let i = 0;
  const swap = () => {
    rect1.left.set(positions[++i % 2].left);
    rect1.right.set(positions[i % 2].right);
    rect1.top.set(positions[i % 2].top);
    rect1.bottom.set(positions[i % 2].bottom);
    rect2.left.set(positions[(i+1) % 2].left);
    rect2.right.set(positions[(i+1) % 2].right);
    rect2.top.set(positions[(i+1) % 2].top);
    rect2.bottom.set(positions[(i+1) % 2].bottom);
  }

  rect1.events.click.addListener(swap);
  rect2.events.click.addListener(swap);

  frame.children.append(rect1);
  frame.children.append(rect2);
  frame.children.append(rect3);

  frame.children.append(dom.Rect({
    pos: {
      left: new Binding(Anchor.fromStart()),
      right: new Binding(Anchor.fromEnd()),
      top: new Binding(Anchor.fromStart()),
      bottom: model.headerHeight,
    },
    fill: new Binding(Color.rgb(0.5, 0.5, 0.5)),
  }));
}