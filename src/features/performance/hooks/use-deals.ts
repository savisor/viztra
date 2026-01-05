import { useState, useEffect, useCallback } from "react";
import { invokeCommand } from "@/shared/services/tauri";
import type { Deal } from "@/features/deals/types";

/**
 * Hook for retrieving deals/trades from a Parquet file by account number
 * @param accountNumber - The account number (filename without .parquet extension), defaults to "5043757397"
 * @param autoFetch - If true, automatically fetches data on mount
 * @returns Object containing deals array, loading state, error state, and fetch function
 */
interface UseDealsReturn {
  deals: Deal[];
  isLoading: boolean;
  error: string | null;
  fetchDeals: (overrideAccountNumber?: string) => Promise<void>;
  accountNumber: string;
}

export function useDeals(
  accountNumber: string = "5043757397",
  autoFetch: boolean = true
): UseDealsReturn {
  const [deals, setDeals] = useState<Deal[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [currentAccountNumber, setCurrentAccountNumber] = useState(accountNumber);

  const fetchDeals = useCallback(
    async (overrideAccountNumber?: string): Promise<void> => {
      const targetAccount = overrideAccountNumber ?? currentAccountNumber;

      if (!targetAccount.trim()) {
        setError("Account number is required");
        return;
      }

      setIsLoading(true);
      setError(null);
      setDeals([]);

      try {
        // Ensure filename ends with .parquet
        const filename = targetAccount.endsWith(".parquet")
          ? targetAccount
          : `${targetAccount}.parquet`;

        const result = await invokeCommand<Deal[]>("read_deals_from_file", {
          filename,
        });
        setDeals(result);
        setCurrentAccountNumber(targetAccount);
      } catch (err) {
        const errorMessage =
          err instanceof Error
            ? err.message
            : "Failed to retrieve deals";
        setError(errorMessage);
        setDeals([]);
      } finally {
        setIsLoading(false);
      }
    },
    [currentAccountNumber]
  );

  // Auto-fetch on mount if autoFetch is enabled
  useEffect(() => {
    if (autoFetch && accountNumber) {
      fetchDeals();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [autoFetch, accountNumber]);

  return {
    deals,
    isLoading,
    error,
    fetchDeals,
    accountNumber: currentAccountNumber,
  };
}

