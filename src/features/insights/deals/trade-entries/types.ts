/**
 * Types for trade_entries insight
 */

import type { Deal } from "@/features/deals/types";

export interface TradeEntriesParams {
  account_number?: string;
}

// Result type is the existing Deal type
export type TradeEntriesResult = Deal;

