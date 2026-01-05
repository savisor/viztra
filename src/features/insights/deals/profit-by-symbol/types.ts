/**
 * Types for profit_by_symbol insight
 */

export interface ProfitBySymbolParams {
  account_number?: string;
  min_profit?: number;
}

export interface ProfitBySymbolResult {
  symbol: string;
  total_profit: number;
  total_volume: number;
  trade_count: number;
  avg_profit: number;
}

