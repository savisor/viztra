/**
 * Types for all_entries insight
 */

import type { Deal } from "@/features/deals/types";

export interface AllEntriesParams {
  account_number?: string;
}

// Result type is the existing Deal type
export type AllEntriesResult = Deal;

