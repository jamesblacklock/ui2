import { Dom, Container, Length } from './dom';

import Layouts from './dist/layouts';

export default function(dom: Dom, frame: Container) {
  const item = new Layouts(dom);
  (window as any).item = item;
  frame.children.append(item);
}
