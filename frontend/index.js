const { Dom } = require('../html-dom');
const render = require('./app');
const dom = new Dom(document);
render(dom, dom.body);

Object.assign(window, require('../html-dom'));
window.dom = dom;
window.rect1 = dom.body.children.get(0);
window.rect2 = dom.body.children.get(1);
window.rect3 = dom.body.children.get(2);
