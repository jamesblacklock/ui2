import * as Dom from "./dom";
export default class Layouts
  extends Dom.Container<Dom.Rect>
{
  #publicModel = new Dom.Model({
    padding: new Dom.Binding(Dom.Length),
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
      var c = dom.Layout();
      ((e) => {
        e.bindings.padding.connect([this.bindings.padding]);
        e.props.layout = Dom.Enum.Layout.Row;
        var c = dom.Pane();
        ((e) => {
          var c = dom.Rect();
          ((e) => {
            e.props.fill = Dom.Brush.rgb(Math.random(), Math.random(), Math.random());
          })(c);
          e.children.append(c);
        })(c);
        e.children.append(c);
        var c = dom.Pane();
        ((e) => {
          var c = dom.Rect();
          ((e) => {
            e.props.fill = Dom.Brush.rgb(Math.random(), Math.random(), Math.random());
          })(c);
          e.children.append(c);
        })(c);
        e.children.append(c);
        var c = dom.Pane();
        ((e) => {
          var c = dom.Rect();
          ((e) => {
            e.props.fill = Dom.Brush.rgb(Math.random(), Math.random(), Math.random());
          })(c);
          e.children.append(c);
        })(c);
        e.children.append(c);
        var c = dom.Pane();
        ((e) => {
          var c = dom.Rect();
          ((e) => {
            e.props.fill = Dom.Brush.rgb(Math.random(), Math.random(), Math.random());
          })(c);
          e.children.append(c);
        })(c);
        e.children.append(c);
        var c = dom.Pane();
        ((e) => {
          var c = dom.Rect();
          ((e) => {
            e.props.fill = Dom.Brush.rgb(Math.random(), Math.random(), Math.random());
          })(c);
          e.children.append(c);
        })(c);
        e.children.append(c);
        var c = dom.Pane();
        ((e) => {
          var c = dom.Rect();
          ((e) => {
            e.props.fill = Dom.Brush.rgb(Math.random(), Math.random(), Math.random());
          })(c);
          e.children.append(c);
        })(c);
      })(c);
      const layouts = [Dom.Enum.Layout.Column, Dom.Enum.Layout.Row];
      let qq = 0;
      e.events.pointer.click.addListener(() => c.props.layout = layouts[qq++ % 2]);
      e.children.append(c);
    })(this.root);
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
    this.root.inject(deps);
  }
}
