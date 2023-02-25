import { Dom, Container, Int, Length } from './dom';

import Layouts from './dist/layouts';

export default function(dom: Dom, frame: Container) {
  const item = new Layouts(dom);
  item.root.events.pointer.click.addListener(() => {
    item.props.rows = Int.from(80);
    item.props.columns = Int.from(80);
  });
  (window as any).item = item;
  frame.children.append(item);
}
