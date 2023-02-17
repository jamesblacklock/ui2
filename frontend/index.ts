import { Dom } from './html-dom';
import render from './app';

const dom = new Dom(document);
render(dom, dom.body);

Object.assign(window, require('./html-dom'));
(window as any).dom = dom;
(window as any).rect1 = dom.body.children.get(0);
(window as any).rect2 = dom.body.children.get(1);
(window as any).rect3 = dom.body.children.get(2);
