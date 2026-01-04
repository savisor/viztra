/**
 * Chart feature types
 */

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
