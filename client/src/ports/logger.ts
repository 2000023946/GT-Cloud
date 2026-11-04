export interface Logger {
    info(message: string, fields?: Record<string, unknown>): void;
    error(message: string, fields?: Record<string, unknown>): void;
}