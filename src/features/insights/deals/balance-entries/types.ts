/**
 * Types for balance_entries insight
 */

import type { Deal } from "@/features/deals/types";

export interface BalanceEntriesParams {
  account_number?: string;
}

// Result type is the existing Deal type
export type BalanceEntriesResult = Deal;

