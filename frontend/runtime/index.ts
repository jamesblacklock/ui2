import { loadRuntime } from '../../runtime/dist/runtime_wasm';

type FinalizationData = { ptr: number; drop: (ptr: number) => void };

interface PropertyMethodTable<Get = number, Set = Get> {
  new: (ptr: number, notify: number) => number;
  drop: (ptr: number) => void;
  get: (ptr: number) => Get;
  set: (ptr: number, value: Set, resptr: number) => void;
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
  property__int__set: PropertyMethodTable<bigint, bigint>['set'];
  property__int__unbind: PropertyMethodTable['unbind'];
  property__int__weakref: PropertyMethodTable['weakref'];
  property__string__bind: PropertyMethodTable['bind'];
  property__string__drop: PropertyMethodTable['drop'];
  property__string__freeze: PropertyMethodTable['freeze'];
  property__string__get: PropertyMethodTable['get'];
  property__string__set: PropertyMethodTable['set'];
  property__string__unbind: PropertyMethodTable['unbind'];
  property__string__weakref: PropertyMethodTable['weakref'];
  property__weakref__drop(ptr: number): void;
  property_factory__commit_changes(ptr: number): number;
  property_factory__drop(ptr: number): void;
  property_factory__new_factory(): number;
  property_factory__new_property__boolean(ptr: number, notify: number): number;
  property_factory__new_property__float(ptr: number, notify: number): number;
  property_factory__new_property__int(ptr: number, notify: number): number;
  property_factory__new_property__string(ptr: number, notify: number): number;
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
  wrapped_value__wrap_int(ptr: bigint): number;
  wrapped_value__unwrap_int(ptr: number): number;
  wrapped_value__wrap_boolean(ptr: number): number;
  wrapped_value__unwrap_boolean(ptr: number): number;
  wrapped_value__wrap_float(ptr: number): number;
  wrapped_value__unwrap_float(ptr: number): number;
  wrapped_value__wrap_string(ptr: number): number;
  wrapped_value__unwrap_string(ptr: number): number;
  wrapped_value__drop(ptr: number): void;
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
  set: (ptr, value, resptr) => ABI.property__int__set(ptr, BigInt(value), resptr),
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
  new: ABI.property_factory__new_property__float,
  drop: ABI.property__float__drop,
  get: ABI.property__float__get,
  set: ABI.property__float__set,
  weakref: ABI.property__float__weakref,
  freeze: ABI.property__float__freeze,
  bind: ABI.property__float__bind,
  unbind: ABI.property__float__unbind,
};
const BOOLEAN_TABLE: PropertyMethodTable<boolean> = {
  new: ABI.property_factory__new_property__boolean,
  drop: ABI.property__boolean__drop,
  get: ptr => ABI.property__boolean__get(ptr) != 0,
  set: (ptr, value, resptr) => ABI.property__boolean__set(ptr, Number(value), resptr),
  weakref: ABI.property__boolean__weakref,
  freeze: ABI.property__boolean__freeze,
  bind: ABI.property__boolean__bind,
  unbind: ABI.property__boolean__unbind,
};

export class PropertyFactory {
  private ptr: number;
  private commitChangesRequested: number|undefined = undefined;

  constructor() {
    this.ptr = ABI.property_factory__new_factory();
    FINALIZER.register(this, {ptr: this.ptr, drop: ABI.property_factory__drop});
  }
  int(notify: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = INT_TABLE;
    return Property.int(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  string(notify: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = STRING_TABLE;
    return Property.string(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  float(notify: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = FLOAT_TABLE;
    return Property.float(this, table.new.call(null, this.ptr, notifyPtr), table);
  }
  boolean(notify: () => unknown) {
    let notifyPtr = notify ? addToHeap(notify) : 0;
    let table = BOOLEAN_TABLE;
    return Property.boolean(this, table.new.call(null, this.ptr, notifyPtr), table);
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

class Property<Get, Set = Get> {
  static int(factory: PropertyFactory, ptr: number, table: PropertyMethodTable<number>) {
    return new Property(factory, ptr, table);
  }
  static string(factory: PropertyFactory, ptr: number, table: PropertyMethodTable<string>) {
    return new Property<string>(factory, ptr, table);
  }
  static boolean(factory: PropertyFactory, ptr: number, table: PropertyMethodTable<boolean>) {
    return new Property<boolean>(factory, ptr, table);
  }
  static float(factory: PropertyFactory, ptr: number, table: PropertyMethodTable<number>) {
    return new Property(factory, ptr, table);
  }

  private result: AbiResult;

  constructor(private factory: PropertyFactory, private ptr: number, private table: PropertyMethodTable<Get, Set>) {
    this.result = AbiResult.new();
    FINALIZER.register(this, { ptr: this.ptr, drop: table.drop });
  }
  get() {
    return this.table.get.call(null, this.ptr);
  }
  set(value: Set) {
    this.table.set.call(null, this.ptr, value, this.result.ptr);
    this.result.verify();
    this.factory.commitChanges();
  }
  freeze() {
    this.table.freeze.call(null, this.ptr);
  }
  weakref() {
    return PropertyWeakRef.fromRuntimePtr(this.table.weakref.call(null, this.ptr));
  }
  bind<
    P extends readonly Property<any>[],
    PV extends { [Q in keyof P]: P[Q] extends Property<infer X, infer Y> ? X : never },
  >(parents: P, fn: (values: PV) => Get) {
    const vec = PropertyVec.new();
    for(const parent of parents) {
      vec.push(parent);
    }
    const wrapperFn = (argsptr: number) => {
      const args = ValueVec.fromRuntimePtr(argsptr);
      const values = args.toArray().map(e => e.unwrap());
      let result = fn(values as unknown as PV);
      const wrapped = WrappedValue.wrap(result as string | number).ptr;
      return wrapped;
    };
    this.table.bind.call(null, this.ptr, vec.ptr, addToHeap(wrapperFn), this.result.ptr);
    this.result.verify();
  }
  unbind() {
    this.table.unbind.call(null, this.ptr);
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
};
type VecClass<Get, Push> = typeof Vec<Get, Push>;
class Vec<Get, Push> {
  static _new<Get, Push>(
    vecClass: VecClass<Get, Push>,
    config: VecConfig<Get, Push>,
    drop: (ptr: number) => void,
  ) {
    return new vecClass(config, undefined, drop);
  }
  static _fromRuntimePtr<Get, Push>(
    vecClass: VecClass<Get, Push>,
    config: VecConfig<Get, Push>,
    ptr: number,
    drop: (ptr: number) => void,
  ) {
    return new vecClass(config, ptr, drop);
  }

  readonly ptr: number;
  private fget: (ptr: number, index: number) => number;
  private flen: (ptr: number) => number;
  private fpush: (ptr: number, itemptr: number) => void;
  private ftoptr: (item: Push) => number;
  private ffromptr: (ptr: number) => Get;

  constructor(config: VecConfig<Get, Push>, ptr: number|undefined, drop: (ptr: number) => void) {
    const { fnew, fget, fpush, flen, ftoptr, ffromptr } = config;
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
    }
    return Vec._new(this, config, ABI.vec__weakref_property__drop);
  }
}

class WrappedValue {
  static BOOLEAN = 0;
  static INT = 1;
  static FLOAT = 2;
  static STRING = 1;

  static wrap(value: number|string) {
    let ptr;
    if(typeof value === 'number') {
      if(Math.abs(value) === value) {
        ptr = ABI.wrapped_value__wrap_int(BigInt(value));
      } else {
        ptr = ABI.wrapped_value__wrap_float(value);
      }
    } else if(typeof value === 'string') {
      ptr = ABI.wrapped_value__wrap_string(AbiBuffer.fromString(value).ptr);
    } else {
      throw new Error('unimplemented');
    }
    return new WrappedValue(ptr);
  }
  static fromRuntimePtr(ptr: number) {
    return new WrappedValue(ptr);
  }

  constructor(readonly ptr: number) {}
  unwrap() {
    const tag = this.tag();
    if(tag === WrappedValue.INT) {
      return Number(ABI.wrapped_value__unwrap_int(this.ptr));
    } else if(tag === WrappedValue.STRING) {
      return AbiBuffer.fromRuntimePtr(ABI.wrapped_value__unwrap_string(this.ptr)).toString();
    } else {
      throw new Error('unimplemented');
    }
  }
  tag() {
    return ABI.wrapped_value__tag(this.ptr);
  }
}

class ValueVec extends Vec<WrappedValue, string|number> {
  static fromRuntimePtr(ptr: number) {
    const config = {
      fnew: ABI.vec__wrapped_value__new,
      fpush: ABI.vec__wrapped_value__push,
      fget: ABI.vec__wrapped_value__get,
      flen: ABI.vec__wrapped_value__len,
      ftoptr: (e: string|number) => WrappedValue.wrap(e).ptr,
      ffromptr: (ptr: number) => WrappedValue.fromRuntimePtr(ptr)
    }
    return Vec._fromRuntimePtr(this, config, ptr, ABI.vec__wrapped_value__drop);
  }
}
