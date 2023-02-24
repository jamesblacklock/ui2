import * as Dom from "./dom";
export default class Layouts
  extends Dom.Container<Dom.Rect>
{
  #publicModel = new Dom.Model({
    padding: new Dom.Binding(Dom.Length),
    layout: new Dom.Binding(Dom.Enum.Layout),
    item: new Dom.Binding(Dom.Component),
  });
  #privateModel = new Dom.Model({});
  readonly root: Dom.Rect;
  readonly events: Dom.Rect["events"];
  constructor(dom: Dom.Dom) {
    const root = dom.Rect()
    super(root);
    this.root = root;
    this.events = this.root.events;
    ((e) => {
      e.props.fill = Dom.Brush.rgb(0.8, 0.8, 0.8);
      e.props.x1 = Dom.Length.px(-200);
      e.props.x2 = Dom.Length.px(200);
      e.props.y1 = Dom.Length.px(-200);
      e.props.y2 = Dom.Length.px(200);
      const layouts = [Dom.Enum.Layout.Column, Dom.Enum.Layout.Row];
      let qq = 0;
      e.events.pointer.click.addListener(() => this.props.layout = layouts[qq++ % 2]);
      var c = dom.Layout();
      ((e) => {
        e.bindings.padding.connect([this.bindings.padding]);
        e.bindings.layout.connect([this.bindings.layout]);

        const repeater = dom.Repeater<Dom.Int, Dom.Pane>(Dom.Int, p => {
          var q = dom.Pane();
          var c = dom.Rect();
          c.props.fill = Dom.Brush.rgb(Math.random(), Math.random(), Math.random());
          c.scaleToParent.set(Dom.Float.from(1));
          q.children.append(c);
          return [q];
        });
        e.children.append(repeater);
        (window as any).repeater = repeater;
      })(c);
      e.children.append(c);
    })(this.root);
  }
  get props() {
    return this.#publicModel.props;
  }
  get bindings() {
    return this.#publicModel.bindings;
  }
  getRoots() { return [this.root]; }
  provide() { return {}; }
  inject(deps: { [key: string]: any }) {
    this.root.inject(deps);
  }
}
