/**
 * Types for trade_entries_with_balance insight
 */

import type { Deal } from "@/features/deals/types";

export interface TradeEntriesWithBalanceParams {
  account_number?: string;
}

// Result type is the existing Deal type
export type TradeEntriesWithBalanceResult = Deal;

