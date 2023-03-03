export function Int(n: number): number { return Math.floor(n); }
export function Float(n: number): number { return n; }
export function String(n: number): string { return globalThis.String(n); }
export function Boolean(n: number): boolean { return !!n; }
