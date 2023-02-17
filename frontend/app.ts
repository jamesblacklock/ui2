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
} from './dom';

class Square implements Component<Element>, Container {
  readonly x = new Binding(Length);
  readonly y = new Binding(Length);
  readonly size = new Binding(Length);
  readonly rect: Rect;

  readonly events: Rect['events'];

  constructor(dom: Dom, t?: Transition) {
    this.rect = dom.Rect();
    this.rect.fill.set(Brush.rgb(Math.random(), Math.random(), Math.random()));
    this.rect.x1.connect([this.x, this.size], ([x, size]) => x.sub(size.div(2)), t);
    this.rect.x2.connect([this.x, this.size], ([x, size]) => x.add(size.div(2)), t);
    this.rect.y1.connect([this.y, this.size], ([y, size]) => y.sub(size.div(2)), t);
    this.rect.y2.connect([this.y, this.size], ([y, size]) => y.add(size.div(2)), t);

    this.events = this.rect.events;
  }

  getRoot() { return this.rect; }
  get children() { return this.rect.children }
}


export default function(dom: Dom, frame: Container) {
  const square1 = new Square(dom, new Ease(2000));
  // square1.size.set(Length.px(60));
  square1.size.connect([square1.rect.frameSize.height], p => p[0].mul(0.25));
  square1.x.set(Length.px(-300));
  square1.y.set(Length.px(0));

  const square2 = new Square(dom, new Ease(2000));
  square2.size.connect([square2.rect.frameSize.height], p => p[0].mul(0.5));
  square2.x.set(Length.px(0));
  square2.y.set(Length.px(0));

  const square3 = new Square(dom);
  square3.size.set(Length.px(60));
  square3.x.set(Length.px(-300));
  square3.y.set(Length.px(100));

  square3.getRoot().fill.transition = new Ease(400);
  square3.events.pointer.click.addListener(
    e => e.getRoot().fill.set(Brush.rgb(Math.random(), Math.random(), Math.random())),
    '0',
  );

  setTimeout(() => {
    square1.x.set(Length.px(300));
    let q = 0;
    const interval = setInterval(
      () => {
        const v = square1.getRoot().x1.get(true);
        square3.x.set(v);
        if(++q > 200) {
          clearInterval(interval);
          console.log('done!');
        }
      }, 10);
  },
  1000);

  frame.children.append(square1);
  frame.children.append(square3);
  requestAnimationFrame(() => square1.children.append(square2));
}
