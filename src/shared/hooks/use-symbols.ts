import { useState } from "react";
import { invokeCommand } from "@/shared/lib/tauri";

/**
 * Hook for fetching and managing available symbols
 * @returns Object containing symbols array, loading state, error state, and fetch function
 */
export function useSymbols() {
  const [symbols, setSymbols] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [hasFetched, setHasFetched] = useState(false);

  const fetchSymbols = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await invokeCommand<string[]>("list_symbols");
      console.log("Fetched symbols:", result);
      setSymbols(result);
      setHasFetched(true);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Failed to fetch symbols";
      console.error("Error fetching symbols:", err);
      setError(errorMessage);
      setSymbols([]);
      setHasFetched(true);
    } finally {
      setIsLoading(false);
    }
  };

  return {
    symbols,
    isLoading,
    error,
    hasFetched,
    fetchSymbols,
  };
}

