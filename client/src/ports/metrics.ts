export interface Metrics {
    increment(name: string, value?: number): void;
    observe(name: string, value: number): void;
}