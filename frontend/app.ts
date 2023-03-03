import { Dom, Container, Length, Iter } from './dom';

import Layouts from './dist/layouts';

export default function(dom: Dom, frame: Container) {
  const item = new Layouts(dom);
  item.props.r = Iter.fromInt(3);
  item.events.pointer.click.addListener(() => {
    item.props.rows = 4;
    item.props.columns = 40;
  });
  (window as any).item = item;
  frame.children.append(item);
}
