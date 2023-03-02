import { loadRuntime } from '../../runtime/dist/runtime_wasm';

type FinalizationData = { ptr: number; drop: (ptr: number) => void };

interface PropertyMethodTable<V = number> {
  new: (ptr: number, notify: number) => number;
  drop: (ptr: number) => void;
  get: (ptr: number) => V;
  set: (ptr: number, value: V, resptr: number) => void;
  weakref: (ptr: number) => number;
  freeze: (ptr: number) => void;
  bind: (ptr: number, parents: number, transform: number, resptr: number) => void;
  unbind: (ptr: number) => void;
}

interface Abi {
  memory: WebAssembly.Memory;
  abi_buffer__drop(ptr: number): void;
  abi_buffer__len(ptr: number): number;
  abi_buffer__new(len: number): number;
  abi_buffer__ptr(ptr: number): number;
  abi_result__drop(ptr: number): void;
  abi_result__is_err(ptr: number): 0|1;
  abi_result__is_ok(ptr: number): 0|1;
  abi_result__message(ptr: number): number;
  abi_result__new(): number;
  property__boolean__bind: PropertyMethodTable['bind'];
  property__boolean__drop: PropertyMethodTable['drop'];
  property__boolean__freeze: PropertyMethodTable['freeze'];
  property__boolean__get: PropertyMethodTable['get'];
  property__boolean__set: PropertyMethodTable['set'];
  property__boolean__unbind: PropertyMethodTable['unbind'];
  property__boolean__weakref: PropertyMethodTable['weakref'];
  property__float__bind: PropertyMethodTable['bind'];
  property__float__drop: PropertyMethodTable['drop'];
  property__float__freeze: PropertyMethodTable['freeze'];
  property__float__get: PropertyMethodTable['get'];
  property__float__set: PropertyMethodTable['set'];
  property__float__unbind: PropertyMethodTable['unbind'];
  property__float__weakref: PropertyMethodTable['weakref'];
  property__int__bind: PropertyMethodTable['bind'];
  property__int__drop: PropertyMethodTable['drop'];
  property__int__freeze: PropertyMethodTable['freeze'];
  property__int__get: PropertyMethodTable['get'];
  property__int__set: PropertyMethodTable['set'];
  property__int__unbind: PropertyMethodTable['unbind'];
  property__int__weakref: PropertyMethodTable['weakref'];
  property__string__bind: PropertyMethodTable['bind'];
  property__string__drop: PropertyMethodTable['drop'];
  property__string__freeze: PropertyMethodTable['freeze'];
  property__string__get: PropertyMethodTable['get'];
  property__string__set: PropertyMethodTable['set'];
  property__string__unbind: PropertyMethodTable['unbind'];
  property__string__weakref: PropertyMethodTable['weakref'];
  property__length__bind: PropertyMethodTable['bind'];
  property__length__drop: PropertyMethodTable['drop'];
  property__length__freeze: PropertyMethodTable['freeze'];
  property__length__get: PropertyMethodTable['get'];
  property__length__set: PropertyMethodTable['set'];
  property__length__unbind: PropertyMethodTable['unbind'];
  property__length__weakref: PropertyMethodTable['weakref'];
  property__brush__bind: PropertyMethodTable['bind'];
  property__brush__drop: PropertyMethodTable['drop'];
  property__brush__freeze: PropertyMethodTable['freeze'];
  property__brush__get: PropertyMethodTable['get'];
  property__brush__set: PropertyMethodTable['set'];
  property__brush__unbind: PropertyMethodTable['unbind'];
  property__brush__weakref: PropertyMethodTable['weakref'];
  property__enum_layout__bind: PropertyMethodTable['bind'];
  property__enum_layout__drop: PropertyMethodTable['drop'];
  property__enum_layout__freeze: PropertyMethodTable['freeze'];
  property__enum_layout__get: PropertyMethodTable['get'];
  property__enum_layout__set: PropertyMethodTable['set'];
  property__enum_layout__unbind: PropertyMethodTable['unbind'];
  property__enum_layout__weakref: PropertyMethodTable['weakref'];
  property__iter__bind: PropertyMethodTable['bind'];
  property__iter__drop: PropertyMethodTable['drop'];
  property__iter__freeze: PropertyMethodTable['freeze'];
  property__iter__get: PropertyMethodTable['get'];
  property__iter__set: PropertyMethodTable['set'];
  property__iter__unbind: PropertyMethodTable['unbind'];
  property__iter__weakref: PropertyMethodTable['weakref'];
  property__weakref__drop(ptr: number): void;
  property_factory__commit_changes(ptr: number): number;
  property_factory__drop(ptr: number): void;
  property_factory__new_factory(): number;
  property_factory__new_property__boolean(ptr: number, notify: number): number;
  property_factory__new_property__float(ptr: number, notify: number): number;
  property_factory__new_property__int(ptr: number, notify: number): number;
  property_factory__new_property__string(ptr: number, notify: number): number;
  property_factory__new_property__length(ptr: number, notify: number): number;
  property_factory__new_property__brush(ptr: number, notify: number): number;
  property_factory__new_property__enum_layout(ptr: number, notify: number): number;
  property_factory__new_property__iter(ptr: number, notify: number): number;
  vec__weakref_property__drop(ptr: number): void;
  vec__weakref_property__get(ptr: number): number;
  vec__weakref_property__len(ptr: number): number;
  vec__weakref_property__new(): number;
  vec__weakref_property__push(ptr: number): number;
  vec__wrapped_value__drop(ptr: number): void;
  vec__wrapped_value__get(ptr: number): number;
  vec__wrapped_value__len(ptr: number): number;
  vec__wrapped_value__new(): number;
  vec__wrapped_value__push(ptr: number): number;
  wrapped_value__tag(ptr: number): number;
  wrapped_value__wrap_int(ptr: number): number;
  wrapped_value__unwrap_int(ptr: number): number;
  wrapped_value__wrap_boolean(ptr: number): number;
  wrapped_value__unwrap_boolean(ptr: number): number;
  wrapped_value__wrap_float(ptr: number): number;
  wrapped_value__unwrap_float(ptr: number): number;
  wrapped_value__wrap_string(ptr: number): number;
  wrapped_value__unwrap_string(ptr: number): number;
  wrapped_value__wrap_length(ptr: number): number;
  wrapped_value__unwrap_length(ptr: number): number;
  wrapped_value__wrap_brush(ptr: number): number;
  wrapped_value__unwrap_brush(ptr: number): number;
  wrapped_value__wrap_enum_layout(ptr: number): number;
  wrapped_value__unwrap_enum_layout(ptr: number): number;
  wrapped_value__drop(ptr: number): void;
  length__px(value: number): number;
  length__cm(value: number): number;
  length__in(value: number): number;
  length__html_vw(value: number): number;
  length__html_vh(value: number): number;
  length__add(lptr: number, rptr: number): number;
  length__sub(lptr: number, rptr: number): number;
  length__mul(ptr: number, value: number): number;
  length__div(ptr: number, value: number): number;
  length__neg(ptr: number): number;
  length__to_string(ptr: number): number;
  length__drop(ptr: number): void;
  brush__rgba(r: number, g: number, b: number, a: number): number;
  brush__to_string(ptr: number): number;
  brush__drop(ptr: number): void;
  iter__from_int(value: number): number;
  iter__to_string(ptr: number): number;
  iter__next(ptr: number): number
  iter__drop(ptr: number): void;
}

const wasm_imports = {
  runtime: {
    __dispatch_function(ptr: number, args: number) {
      const fn = getHeapObject(ptr);
      return fn(args);
    },
    __drop_function(ptr: number) {
      dropFromHeap(ptr);
    },
    __console_log(ptr: number, isError: 0|1) {
      const message = AbiBuffer.fromRuntimePtr(ptr).toString();
      if(isError) {
        console.error(message);
      } else {
        console.log(message);
      }
    },
  },
};

const WASM = await loadRuntime(wasm_imports);
const ABI: Abi = WASM.instance.exports as unknown as Abi;

const HEAP: Function[] = [() => null];
const FREE_HEAP: number[] = [];
const DECODER = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
const ENCODER = new TextEncoder();
const FINALIZER = new FinalizationRegistry(({ drop, ptr }: FinalizationData) => drop(ptr));

function addToHeap(item: Function) {
  let ptr = FREE_HEAP.pop();
  if(ptr === undefined) {
    ptr = HEAP.length;
    HEAP.push(item);
  } else {
    HEAP[ptr] = item;
  }
  return ptr;
}
function dropFromHeap(ptr: number) {
  let result = HEAP[ptr];
  if(ptr > 2) {
    delete HEAP[ptr];
    FREE_HEAP.push(ptr);
  }
  return result;
}
function getHeapObject(ptr: number) {
  return HEAP[ptr];
}


class AbiBuffer {
  static fromRuntimePtr(ptr: number) {
    return new AbiBuffer(ptr);
  }
  static fromString(string: string) {
    return new AbiBuffer(undefined, string);
  }

  readonly ptr: number;
  readonly len: number;
  private buf: number;

  constructor(ptr: number|undefined, string?: string) {
    if(ptr) {
      this.ptr = ptr;
      this.buf = ABI.abi_buffer__ptr(this.ptr);
      this.len = ABI.abi_buffer__len(this.ptr);
    } else {
      const encoded = ENCODER.encode(string);
      this.ptr = ABI.abi_buffer__new(encoded.length);
      this.buf = ABI.abi_buffer__ptr(this.ptr);
      this.len = ABI.abi_buffer__len(this.ptr);

      const buffer = new Uint8Array(ABI.memory.buffer).subarray(this.buf, this.buf + encoded.length);
      buffer.set(encoded);
    }
    FINALIZER.register(this, {ptr: this.ptr, drop: ABI.abi_buffer__drop});
  }
  toString() {
    if(this.len === 0) {
      return '';
    }
    const buffer = new Uint8Array(ABI.memory.buffer).subarray(this.buf, this.buf + this.len);
    return DECODER.decode(buffer);
  }
}

class AbiResult {
  static new() {
    return new AbiResult();
  }

  readonly ptr: number;

  constructor() {
    this.ptr =  ABI.abi_result__new();
    FINALIZER.register(this, {ptr: this.ptr, drop: ABI.abi_result__drop});
  }
  isOk() {
    return !!ABI.abi_result__is_ok(this.ptr);
  }
  isErr() {
    return !!ABI.abi_result__is_err(this.ptr);
  }
  get message() {
    let buf = AbiBuffer.fromRuntimePtr(ABI.abi_result__message(this.ptr));
    return buf.toString();
  }
  verify() {
    if(this.isErr()) {
      throw new Error(this.message);
    }
  }
}

const INT_TABLE: PropertyMethodTable<number> = {
  new: ABI.property_factory__new_property__int,
  drop: ABI.property__int__drop,
  get: ptr => Number(ABI.property__int__get(ptr)),
  set: (ptr, value, resptr) => ABI.property__int__set(ptr, value, resptr),
  weakref: ABI.property__int__weakref,
  freeze: ABI.property__int__freeze,
  bind: ABI.property__int__bind,
  unbind: ABI.property__int__unbind,
};
const STRING_TABLE: PropertyMethodTable<string> = {
  new: ABI.property_factory__new_property__string,
  drop: ABI.property__string__drop,
  get: ptr => AbiBuffer.fromRuntimePtr(ABI.property__string__get(ptr)).toString(),
  set: (ptr, value, resptr) => ABI.property__string__set(ptr, AbiBuffer.fromString(value).ptr, resptr),
  weakref: ABI.property__string__weakref,
  freeze: ABI.property__string__freeze,
  bind: ABI.property__string__bind,
  unbind: ABI.property__string__unbind,
};
const FLOAT_TABLE: PropertyMethodTable<number> = {
  new: ABI.property_factory__new_property__float,  drop: ABI.property__float__drop,
  get: ABI.property__float__get,
  set: ABI.property__float__set,
  weakref: ABI.property__float__weakref,
  freeze: ABI.property__float__freeze,
  bind: ABI.property__float__bind,
  unbind: ABI.property__float__unbind,
};
const BOOLEAN_TABLE: PropertyMethodTable<boolean> = {
  new: ABI.property_factory__new_property__boolean,  drop: ABI.property__boolean__drop,
  get: ptr => ABI.property__boolean__get(ptr) != 0,
  set: (ptr, value, resptr) => ABI.property__boolean__set(ptr, Number(value), resptr),
  weakref: ABI.property__boolean__weakref,
  freeze: ABI.property__boolean__freeze,
  bind: ABI.property__boolean__bind,
  unbind: ABI.property__boolean__unbind,
};
const LENGTH_TABLE: PropertyMethodTable<Length> = {
  new: ABI.property_factory__new_property__length,  drop: ABI.property__length__drop,
  get: ptr => Length.__fromRuntimePtr(ABI.property__length__get(ptr)),
  set: (ptr, value, resptr) => ABI.property__length__set(ptr, value.ptr, resptr),
  weakref: ABI.property__length__weakref,
  freeze: ABI.property__length__freeze,
  bind: ABI.property__length__bind,
  unbind: ABI.property__length__unbind,
};
const BRUSH_TABLE: PropertyMethodTable<Brush> = {
  new: ABI.property_factory__new_property__brush,  drop: ABI.property__brush__drop,
  get: ptr => Brush.__fromRuntimePtr(ABI.property__brush__get(ptr)),
  set: (ptr, value, resptr) => ABI.property__brush__set(ptr, value.ptr, resptr),
  weakref: ABI.property__brush__weakref,
  freeze: ABI.property__brush__freeze,
  bind: ABI.property__brush__bind,
  unbind: ABI.property__brush__unbind,
};
const ENUM_LAYOUT_TABLE: PropertyMethodTable<Enum.Layout> = {
  new: ABI.property_factory__new_property__enum_layout,  drop: ABI.property__enum_layout__drop,
  get: ptr => Enum.Layout.__fromInt(ABI.property__enum_layout__get(ptr)),
  set: (ptr, value, resptr) => ABI.property__enum_layout__set(ptr, value.value, resptr),
  weakref: ABI.property__enum_layout__weakref,
  freeze: ABI.property__enum_layout__freeze,
  bind: ABI.property__enum_layout__bind,
  unbind: ABI.property__enum_layout__unbind,
};
const ITER_TABLE: PropertyMethodTable<Iter> = {
  new: ABI.property_factory__new_property__iter,  drop: ABI.property__iter__drop,
  get: ptr => Iter.__fromRuntimePtr(ABI.property__iter__get(ptr)),
  set: (ptr, value, resptr) => ABI.property__iter__set(ptr, value.ptr, resptr),
  weakref: ABI.property__iter__weakref,
  freeze: ABI.property__iter__freeze,
  bind: ABI.property__iter__bind,
  unbind: ABI.property__iter__unbind,
};

class PropertyFactoryClass {
  private ptr: number;
  private commitChangesRequested: number|undefined = undefined;

  constructor() {
    this.ptr = ABI.property_factory__new_factory();
    FINALIZER.register(this, {ptr: this.ptr, drop: ABI.property_factory__drop});
  }
  int(notify?: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = INT_TABLE;
    return new Property(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  string(notify?: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = STRING_TABLE;
    return new Property(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  float(notify?: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = FLOAT_TABLE;
    return new Property(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  boolean(notify?: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = BOOLEAN_TABLE;
    return new Property(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  length(notify?: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = LENGTH_TABLE;
    return new Property(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  brush(notify?: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = BRUSH_TABLE;
    return new Property(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  layout(notify?: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = ENUM_LAYOUT_TABLE;
    return new Property(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  iter(notify?: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = ITER_TABLE;
    return new Property(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  commitChanges() {
    if(this.commitChangesRequested === undefined) {
      this.commitChangesRequested = requestAnimationFrame(() => {
        this.commitChangesRequested = undefined
        ABI.property_factory__commit_changes(this.ptr);
      });
    }
  }
}

export const PropertyFactory = new PropertyFactoryClass();

export type PropertyValues<P> = { [Q in keyof P]: P[Q] extends Property<infer X> ? X : never };
export type Transformer<P, V> = (values: PropertyValues<P>) => V;

export class Property<V = any> {
  private result: AbiResult;

  constructor(private factory: PropertyFactoryClass, private ptr: number, private table: PropertyMethodTable<V>) {
    this.result = AbiResult.new();
    FINALIZER.register(this, { ptr: this.ptr, drop: table.drop });
  }
  get() {
    return this.table.get.call(null, this.ptr);
  }
  set(value: V) {
    this.table.set.call(null, this.ptr, value, this.result.ptr);
    this.result.verify();
    this.factory.commitChanges();
    return this;
  }
  freeze() {
    this.table.freeze.call(null, this.ptr);
  }
  weakref() {
    return PropertyWeakRef.fromRuntimePtr(this.table.weakref.call(null, this.ptr));
  }
  bind<P extends readonly Property<any>[]>(parents: P, fn: Transformer<P, V>) {
    const vec = PropertyVec.new();
    for(const parent of parents) {
      vec.push(parent);
    }
    const wrapperFn = (argsptr: number) => {
      const args = ValueVec.fromRuntimePtr(argsptr);
      const values = args.toArray();
      let result = fn(values as unknown as PropertyValues<P>);
      const wrapped = WrappedValue.wrap(result as string | number).ptr;
      return wrapped;
    };
    this.table.bind.call(null, this.ptr, vec.ptr, addToHeap(wrapperFn), this.result.ptr);
    this.result.verify();
    return this;
  }
  unbind() {
    this.table.unbind.call(null, this.ptr);
  }
  toString() {
    return `Property(${String(this.get())})`;
  }
}

class PropertyWeakRef {
  static fromRuntimePtr(ptr: number) {
    return new PropertyWeakRef(ptr);
  }
  constructor(readonly ptr: number) {
    FINALIZER.register(this, {ptr, drop: ABI.property__weakref__drop});
  }
}

type VecConfig<Get, Push> = {
  fnew: () => number;
  fget: (ptr: number, index: number) => number;
  flen: (ptr: number) => number;
  fpush: (ptr: number, itemptr: number) => void;
  ftoptr: (item: Push) => number;
  ffromptr: (ptr: number) => Get;
  drop: (ptr: number) => void;
};

type VecClass<Get, Push> = typeof Vec<Get, Push>;

class Vec<Get, Push> {
  static _new<Get, Push>(
    vecClass: VecClass<Get, Push>,
    config: VecConfig<Get, Push>,
  ) {
    return new vecClass(config, undefined);
  }
  static _fromRuntimePtr<Get, Push>(
    vecClass: VecClass<Get, Push>,
    config: VecConfig<Get, Push>,
    ptr: number,
  ) {
    return new vecClass(config, ptr);
  }

  readonly ptr: number;
  private fget: (ptr: number, index: number) => number;
  private flen: (ptr: number) => number;
  private fpush: (ptr: number, itemptr: number) => void;
  private ftoptr: (item: Push) => number;
  private ffromptr: (ptr: number) => Get;

  constructor(config: VecConfig<Get, Push>, ptr: number|undefined) {
    const { fnew, fget, fpush, flen, ftoptr, ffromptr, drop } = config;
    this.ptr = ptr ?? fnew();
    this.fget = fget;
    this.flen = flen;
    this.fpush = fpush;
    this.ftoptr = ftoptr;
    this.ffromptr = ffromptr;
    FINALIZER.register(this, {ptr: this.ptr, drop});
  }
  push(item: Push) {
    const ptr = this.ftoptr.call(null, item);
    this.fpush.call(null, this.ptr, ptr);
  }
  len() {
    return this.flen.call(null, this.ptr);
  }
  get(index: number) {
    const ptr = this.fget.call(null, this.ptr, index);
    return this.ffromptr.call(null, ptr);
  }
  toArray() {
    const len = this.flen.call(null, this.ptr);
    const arr = [];
    for(let i=0; i<len; i++) {
      arr.push(this.get(i));
    }
    return arr;
  }
}

class PropertyVec extends Vec<PropertyWeakRef, Property<any>> {
  static new() {
    const config = {
      fnew: ABI.vec__weakref_property__new,
      fpush: ABI.vec__weakref_property__push,
      fget: ABI.vec__weakref_property__get,
      flen: ABI.vec__weakref_property__len,
      ftoptr: (property: Property<any>) => property.weakref().ptr,
      ffromptr: (ptr: number) => PropertyWeakRef.fromRuntimePtr(ptr),
      drop: ABI.vec__weakref_property__drop,
    }
    return Vec._new(this, config);
  }
}

const BOOLEAN = 0;
const INT = 1;
const FLOAT = 2;
const STRING = 3;
const LENGTH = 4;
const BRUSH = 5;
const ENUM_LAYOUT = 6;
const ITER = 7;

type Value = number | string | Length | Brush | Enum.Layout;

class WrappedValue {
  static wrap(value: Value) {
    let ptr;
    if(typeof value === 'number') {
      if(Math.abs(value) === value) {
        ptr = ABI.wrapped_value__wrap_int(value);
      } else {
        ptr = ABI.wrapped_value__wrap_float(value);
      }
    } else if(typeof value === 'string') {
      ptr = ABI.wrapped_value__wrap_string(AbiBuffer.fromString(value).ptr);
    } else if(value instanceof Length) {
      ptr = ABI.wrapped_value__wrap_length(value.ptr);
    } else if(value instanceof Brush) {
      ptr = ABI.wrapped_value__wrap_brush(value.ptr);
    } else if(value instanceof Enum.Layout) {
      ptr = ABI.wrapped_value__wrap_enum_layout(value.value);
    } else {
      throw new Error('unimplemented');
    }
    return new WrappedValue(ptr);
  }
  static fromRuntimePtr(ptr: number) {
    return new WrappedValue(ptr);
  }

  constructor(readonly ptr: number) {
    FINALIZER.register(this, {ptr: this.ptr, drop: ABI.wrapped_value__drop});
  }
  unwrap() {
    const tag = this.tag();
    switch(tag) {
      case INT:
        return ABI.wrapped_value__unwrap_int(this.ptr);
      case BOOLEAN:
        return ABI.wrapped_value__unwrap_boolean(this.ptr) != 0;
      case FLOAT:
        return ABI.wrapped_value__unwrap_float(this.ptr);
      case STRING:
        return AbiBuffer.fromRuntimePtr(ABI.wrapped_value__unwrap_string(this.ptr)).toString();
      case LENGTH:
        return Length.__fromRuntimePtr(ABI.wrapped_value__unwrap_length(this.ptr));
      case BRUSH:
        return Brush.__fromRuntimePtr(ABI.wrapped_value__unwrap_brush(this.ptr));
      case ENUM_LAYOUT:
        return Enum.Layout.__fromInt(ABI.wrapped_value__unwrap_enum_layout(this.ptr));
      default:
        throw new Error('unimplemented');
    }
  }
  tag() {
    return ABI.wrapped_value__tag(this.ptr);
  }
}

export class ValueVec extends Vec<Value, Value> {
  static CONFIG = {
    fnew: ABI.vec__wrapped_value__new,
    fpush: ABI.vec__wrapped_value__push,
    fget: ABI.vec__wrapped_value__get,
    flen: ABI.vec__wrapped_value__len,
    ftoptr: (e: Value) => WrappedValue.wrap(e).ptr,
    ffromptr: (ptr: number) => WrappedValue.fromRuntimePtr(ptr).unwrap(),
    drop: ABI.vec__wrapped_value__drop,
  }
  static fromRuntimePtr(ptr: number) {
    return Vec._fromRuntimePtr(this, ValueVec.CONFIG, ptr);
  }
  static new() {
    return Vec._new(this, ValueVec.CONFIG);
  }
}

export class Length {
  static px(value: number) {
    return new Length(value, ABI.length__px);
  }
  static cm(value: number) {
    return new Length(value, ABI.length__cm);
  }
  static in(value: number) {
    return new Length(value, ABI.length__in);
  }
  static __htmlVw(value: number) {
    return new Length(value, ABI.length__html_vw);
  }
  static __htmlVh(value: number) {
    return new Length(value, ABI.length__html_vh);
  }
  static __fromRuntimePtr(ptr: number) {
    return new Length(ptr);
  }

  readonly ptr: number;

  constructor(ptrOrValue: number, fn?: (value: number) => number) {
    if(fn) {
      this.ptr = fn(ptrOrValue);
    } else {
      this.ptr = ptrOrValue;
    }
    FINALIZER.register(this, {ptr: this.ptr, drop: ABI.length__drop});
  }
  add(rhs: Length) {
    return new Length(ABI.length__add(this.ptr, rhs.ptr));
  }
  sub(rhs: Length) {
    return new Length(ABI.length__sub(this.ptr, rhs.ptr));
  }
  mul(rhs: number) {
    return new Length(ABI.length__mul(this.ptr, rhs));
  }
  div(rhs: number) {
    return new Length(ABI.length__div(this.ptr, rhs));
  }
  neg() {
    return new Length(ABI.length__neg(this.ptr));
  }
  toString() {
    return AbiBuffer.fromRuntimePtr(ABI.length__to_string(this.ptr)).toString();
  }
  valueOf() {
    return this.toString();
  }
}

export class Brush {
  static get RED() { return Brush.rgba(1, 0, 0, 1); }
  static get GREEN() { return Brush.rgba(0, 1, 0, 1); }
  static get BLUE() { return Brush.rgba(0, 0, 1, 1); }
  static get WHITE() { return Brush.rgba(1, 1, 1, 1); }
  static get BLACK() { return Brush.rgba(0, 0, 0, 1); }
  static get TRANSPARENT() { return Brush.rgba(0, 0, 0, 0); }

  static rgba(r: number, g: number, b: number, a: number) {
    return new Brush(ABI.brush__rgba(r, g, b, a));
  }
  static __fromRuntimePtr(ptr: number) {
    return new Brush(ptr);
  }

  constructor(readonly ptr: number) {
    FINALIZER.register(this, {ptr: this.ptr, drop: ABI.brush__drop});
  }
  toString() {
    return AbiBuffer.fromRuntimePtr(ABI.brush__to_string(this.ptr)).toString();
  }
  valueOf() {
    return this.toString();
  }
}

export namespace Enum {
  export class Layout {
    static Row = new Layout(0);
    static Column = new Layout(1);
    static __fromInt(value: number) {
      return value === Layout.Column.value ? Layout.Column : Layout.Row;
    }
    constructor(readonly value: number) {
      if(Layout.Column !== undefined) {
        throw new Error('cannot construct new instances of Enum.Layout');
      }
    }
    toString() {
      return this === Layout.Column ? '.column' : '.row';
    }
  }
}

export class Iter {
  static fromInt(value: number) {
    return new Iter(ABI.iter__from_int(value));
  }
  static __fromRuntimePtr(ptr: number) {
    return new Iter(ptr);
  }

  constructor(readonly ptr: number) {
    FINALIZER.register(this, {ptr: this.ptr, drop: ABI.iter__drop});
  }

  toString() {
    return AbiBuffer.fromRuntimePtr(ABI.iter__to_string(this.ptr)).toString();
  }
  
  *[Symbol.iterator]() {
    let ptr = ABI.iter__next(this.ptr);
    while(ptr !== 0) {
      const value = WrappedValue.fromRuntimePtr(ptr);
      yield value.unwrap();
      ptr = ABI.iter__next(this.ptr);
    }
  }
}
