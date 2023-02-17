
export interface Transition {
  interpolate(startTime: number): number;
  time: number
}

export class CubicBezier implements Transition {
  sampleCount = 700;
  samples: Map<number, number> = new Map([[0, 0], [this.sampleCount, 1]]);
  constructor(public time: number, public x1: number, public y1: number, public x2: number, public y2: number) {
    let t = 1/this.sampleCount;
    while(t < 1) {
      const sample = this.sample(t);
      this.samples.set(Math.round(sample.x * this.sampleCount), sample.y);
      t += 1/this.sampleCount;
    }
  }

  sample(t: number): { x: number, y: number } {
    const p0 = { x: 0, y: 0 };
    const p1 = { x: this.x1, y: this.y1 };
    const p2 = { x: this.x2, y: this.y2 };
    const p3 = { x: 1, y: 1 };

    const cX = 3 * (p1.x - p0.x);
    const bX = 3 * (p2.x - p1.x) - cX;
    const aX = p3.x - p0.x - cX - bX;

    const cY = 3 * (p1.y - p0.y);
    const bY = 3 * (p2.y - p1.y) - cY;
    const aY = p3.y - p0.y - cY - bY;
          
    const x = (aX * Math.pow(t, 3)) + (bX * Math.pow(t, 2)) + (cX * t) + p0.x;
    const y = (aY * Math.pow(t, 3)) + (bY * Math.pow(t, 2)) + (cY * t) + p0.y;
    
    return { x, y };
  }

  interpolate(startTime: number): number {
    const elapsed = Date.now() - startTime;
    const progress = elapsed / this.time;
    if(progress === 0) {
      return 0;
    } else if(progress >= 1) {
      return 1;
    }

    let x = progress * this.sampleCount;
    let x1 = Math.round(x);
    let y1 = this.samples.get(x1);
    while(y1 === undefined) {
      y1 = this.samples.get(Math.max(0, Math.floor(--x1)));
    }
    let x2 = Math.round(x + 1);
    let y2 = this.samples.get(x2);
    while(y2 === undefined) {
      y2 = this.samples.get(Math.min(this.sampleCount, Math.floor(++x2)));
    }
    return (x-x1) / (x2-x1) * (y2-y1) + y1;
  }
}

export class Ease extends CubicBezier {
  constructor(public time: number) {
    super(time, 0.25, 0.1, 0.25, 1.0);
  }
}
