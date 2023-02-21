import { Property } from './common';
import { Float } from './types';

export abstract class Length implements Property {
  static px(value: number) { return new Px(value); };
  static default() { return new Px(0) as Length; }
  static coerce(e: any) {
    if(e instanceof Length) {
      return e;
    } else {
      return Length.px(Float.coerce(e).value);
    }
  }

  constructor(public units?: string) {}

  abstract equals(other: Length): boolean;

  addOrSub(other: Length, fac = 1): Length {
    if(this instanceof Px) {
      if(this.value === 0) {
        return fac < 0 ? other.neg() : other;
      } else if(other instanceof Px) {
        return new Px(this.value + other.value * fac);
      }
    }
    if(fac < 0) {
      return new LengthSubtraction(this, other);
    }
    return new LengthAddition(this, other);
  }
  add(other: Length) {
    return this.addOrSub(other);
  }
  sub(other: Length) {
    return this.addOrSub(other, -1);
  }
  mul(fac: number): Length {
    if(fac === 1) {
      return this;
    } else if(fac === 0) {
      return new Px(0);
    }
    if(this instanceof Px) {
      return new Px(this.value * fac);
    }
    return new LengthMultiplication(this, fac);
  }
  div(fac: number): Length {
    if(fac === 1) {
      return this;
    }
    if(this instanceof Px) {
      return new Px(this.value / fac);
    }
    return new LengthDivision(this, fac);
  }
  neg(): Length {
    if(this instanceof Px) {
      return new Px(this.value === 0 ? 0 : -this.value);
    }
    return new LengthNegation(this);
  }

  interpolate(next: Length, fac: number) {
    return (next.sub(this)).mul(fac).add(this) as typeof this;
  }
}

export class LengthAddition extends Length {
  constructor(public op1: Length, public op2: Length) {
    super(op1.units ?? op2.units);
  }
  equals(other: Property) {
    return other instanceof LengthAddition && this.op1.equals(other.op1) && this.op2.equals(other.op2);
  }
}
export class LengthMultiplication extends Length {
  constructor(public op1: Length, public op2: number) {
    super(op1.units);
  }
  equals(other: Property) {
    return other instanceof LengthMultiplication && this.op1.equals(other.op1) && this.op2 === other.op2;
  }
}
export class LengthSubtraction extends Length {
  constructor(public op1: Length, public op2: Length) {
    super(op1.units ?? op2.units);
  }
  equals(other: Property) {
    return other instanceof LengthSubtraction && this.op1.equals(other.op1) && this.op2.equals(other.op2);
  }
}
export class LengthDivision extends Length {
  constructor(public op1: Length, public op2: number) {
    super(op1.units);
  }
  equals(other: Property) {
    return other instanceof LengthDivision && this.op1.equals(other.op1) && this.op2 === other.op2;
  }
}
export class LengthNegation extends Length {
  constructor(public op1: Length) {
    super(op1.units);
  }
  equals(other: Property) {
    return other instanceof LengthNegation && this.op1.equals(other.op1);
  }
}

export class Px extends Length {
  constructor(public value: number) {
    super('px');
  }
  equals(other: Length) {
    return other instanceof Px && other.value === this.value;
  }
  interpolate(next: Length, fac: number) {
    if(next instanceof Px) {
      return new Px((next.value - this.value) * fac + this.value) as typeof this;
    }
    return super.interpolate(next, fac);
  }
  valueOf() {
    return this.value;
  }
}
