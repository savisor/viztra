/**
 * Global TypeScript types and interfaces
 */

// API Response types
export interface ApiResponse<T> {
  data: T;
  error?: string;
  success: boolean;
}

// Common utility types
export type Nullable<T> = T | null;
export type Optional<T> = T | undefined;

// Error types
export interface AppError {
  message: string;
  code?: string;
  details?: unknown;
}

// OHLC (Open, High, Low, Close) data types
export interface OHLCData {
  time: number;  // Unix timestamp in seconds
  open: number;  // Opening price
  high: number;  // Highest price during the interval
  low: number;   // Lowest price during the interval
  close: number; // Closing price
}

// Asset operation result type
export interface AssetOperationResult {
  success: boolean;
  message: string;
}

