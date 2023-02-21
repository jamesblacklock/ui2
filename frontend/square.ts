import * as Dom from "./dom";
export default class Square
  extends Dom.Container<Dom.Rect>
{
  #publicModel = new Dom.Model({
    y: new Dom.Binding(Dom.Length),
    x: new Dom.Binding(Dom.Length),
    size: new Dom.Binding(Dom.Length),
    fizzbuzz: new Dom.Binding(Dom.Float),
    thing: new Dom.Binding(Dom.Component),
  });
  #privateModel = new Dom.Model({
    qwer: new Dom.Binding(Dom.Int),
    parentSize: {
      width: new Dom.Binding(Dom.Length),
      height: new Dom.Binding(Dom.Length),
    }
  });
  readonly root: Dom.Rect;
  readonly events: Dom.Rect["events"];
  constructor(dom: Dom.Dom) {
    const root = dom.Rect();
    super(root);
    this.root = root;
    this.events = this.root.events;
    this.bindings.fizzbuzz.set(Dom.Float.from(420.69)).freeze();
    ((e) => {
      e.props.fill = Dom.Brush.rgb(Math.random(), Math.random(), Math.random());
      e.props.x2 = Dom.Length.px(100);
      e.props.y1 = Dom.Length.px(10);
      e.bindings.x1.connect([this.bindings.x], ([x]) => x);
      e.props.y2 = Dom.Length.px(100);
      var c = dom.Rect();
      ((e) => {
        e.props.fill = Dom.Brush.rgba(
          0.8666666666666667,
          0.6666666666666666,
          0,
          1
        );
        e.props.x1 = Dom.Length.px(-60);
        e.props.x2 = Dom.Length.px(60);
        e.props.y1 = Dom.Length.px(-60);
        e.props.y2 = Dom.Length.px(60);
        var c = dom.Slot();
        ((e) => {
          e.bindings.component.connect([this.bindings.thing]);
        })(c);
        e.children.append(c);
      })(c);
      e.children.append(c);
    })(this.root);

    const t = new Dom.Ease(300);
    this.root.bindings.x1.connect([this.bindings.x, this.bindings.size], ([x, size]) => x.sub(size.div(2)), t);
    this.root.bindings.x2.connect([this.bindings.x, this.bindings.size], ([x, size]) => x.add(size.div(2)), t);
    this.root.bindings.y1.connect([this.bindings.y, this.bindings.size], ([y, size]) => y.sub(size.div(2)), t);
    this.root.bindings.y2.connect([this.bindings.y, this.bindings.size], ([y, size]) => y.add(size.div(2)), t);
  }
  get props() {
    return this.#publicModel.props;
  }
  get bindings() {
    return this.#publicModel.bindings;
  }
  getRoot() {
    return this.root;
  }
  provide() { return {}; }
  inject(deps: { [key: string]: any }) {
    if(deps.frameSize) {
      this.#privateModel.bindings.parentSize.width.connect([deps.frameSize.width]);
      this.#privateModel.bindings.parentSize.height.connect([deps.frameSize.height]);
    }
    this.root.inject(deps);
  }
}
