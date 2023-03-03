
import { PropertyFactory, Length, Brush, Enum, ValueVec, Iter } from './runtime';
import { Dom } from './html-dom';
import render from './app';

const dom = new Dom(document);
render(dom, dom.body);

(window as any).d = require('./html-dom');
(window as any).dom = dom;
(window as any).Enum = Enum;
(window as any).Brush = Brush;
(window as any).ValueVec = ValueVec;
(window as any).Iter = Iter;
(window as any).PropertyFactory = PropertyFactory;
(window as any).Length = Length;
