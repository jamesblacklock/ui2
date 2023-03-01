// import { Dom } from './html-dom';
// import render from './app';

// const dom = new Dom(document);
// render(dom, dom.body);

// (window as any).d = require('./html-dom');
// (window as any).dom = dom;

import { PropertyFactory, Length } from './runtime';

(window as any).PropertyFactory = PropertyFactory;
(window as any).Length = Length;