import {
  Brush,
  Length,
  Binding,
  Ease,
  Rect,
  Element,
  Component,
  Dom,
  Container,
  Transition,
  Model,
} from './dom';

import Square from './dist/square';

/* square.ui 

pub x: Length;
pub y: Length;
pub size: Length;

Rect {
  fill: (.rgb(random(), random(), random()));
  x1: (x - size / 2);
  x2: (x + size / 2);
  y1: (y - size / 2);
  y2: (y + size / 2);
}

*/

// class Square implements Component<Element>, Container {
//   #model = new Model({
//     x: new Binding(Length),
//     y: new Binding(Length),
//     size: new Binding(Length),
//   });

//   readonly root: Rect;
//   readonly events: Rect['events'];

//   constructor(dom: Dom, t?: Transition) {
//     this.root = dom.Rect();
//     this.root.props.fill = Brush.rgb(Math.random(), Math.random(), Math.random());
//     this.root.bindings.x1.connect([this.bindings.x, this.bindings.size], ([x, size]) => x.sub(size.div(2)), t);
//     this.root.bindings.x2.connect([this.bindings.x, this.bindings.size], ([x, size]) => x.add(size.div(2)), t);
//     this.root.bindings.y1.connect([this.bindings.y, this.bindings.size], ([y, size]) => y.sub(size.div(2)), t);
//     this.root.bindings.y2.connect([this.bindings.y, this.bindings.size], ([y, size]) => y.add(size.div(2)), t);

//     this.events = this.root.events;
//   }

//   get props() { return this.#model.props; }
//   get bindings() { return this.#model.bindings; }

//   getRoot() { return this.root; }
//   get children() { return this.root.children }
// }


export default function(dom: Dom, frame: Container) {
  const square1 = new Square(dom);//, new Ease(2000));
  // square1.props.size = Length.px(60);
  square1.bindings.size.connect([square1.root.bindings.frameSize.height], p => p[0].mul(0.25));
  square1.props.x = Length.px(-300);
  square1.props.y = Length.px(0);

  const square2 = new Square(dom);//, new Ease(2000));
  square2.bindings.size.connect([square2.root.bindings.frameSize.height], p => p[0].mul(0.5));
  square2.props.x = Length.px(0);
  square2.props.y = Length.px(0);

  const square3 = new Square(dom);
  square3.props.size = Length.px(60);
  square3.props.x = Length.px(-300);
  square3.props.y = Length.px(100);

  square3.getRoot().bindings.fill.transition = new Ease(400);
  square3.events.pointer.click.addListener(
    e => e.getRoot().props.fill = Brush.rgb(Math.random(), Math.random(), Math.random()),
    '0',
  );

  // setTimeout(() => {
  //   square1.props.x = Length.px(300);
  //   let q = 0;
  //   const interval = setInterval(
  //     () => {
  //       const v = square1.getRoot().bindings.x1.get(true);
  //       square3.props.x = v;
  //       if(++q > 200) {
  //         clearInterval(interval);
  //         console.log('done!');
  //       }
  //     }, 10);
  // },
  // 1000);

  frame.children.append(square1);
  frame.children.append(square3);
  requestAnimationFrame(() => square1.children.append(square2));
}
