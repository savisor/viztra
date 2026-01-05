import { LabelValue } from "@/shared/ui/LabelValue";
import type { BatchInsightItem } from "@/shared/insights/use-batch-insights";
import styles from "./Component.module.css";

interface InsightsGridProps {
  results: BatchInsightItem[];
  isLoading: boolean;
  error: string | null;
}

/**
 * Extracts a display value from an insight result
 * Handles different result types (arrays, single objects, etc.)
 */
const extractValue = (item: BatchInsightItem): React.ReactNode => {
  if (!item.success || !item.data) {
    return item.error || "Error";
  }

  // If data is an array, extract summary metrics
  if (Array.isArray(item.data)) {
    if (item.data.length === 0) {
      return 0;
    }

    const firstItem = item.data[0];

    // For total_balance insight - extract total_balance from first item
    if (item.insight_id === "deals.total_balance" && typeof firstItem.total_balance === "number") {
      return firstItem.total_balance;
    }

    // For profit_by_symbol - sum total_profit from all symbols
    if (item.insight_id === "deals.profit_by_symbol") {
      const totalProfit = item.data.reduce((sum: number, row: Record<string, unknown>) => {
        const profit = typeof row.total_profit === "number" ? row.total_profit : 0;
        return sum + profit;
      }, 0);
      return totalProfit;
    }

    // For insights that return arrays of records (trade_entries, balance_entries, etc.)
    // Return the count of items
    if (item.insight_id.includes("entries") || item.insight_id.includes("_entries")) {
      return item.data.length;
    }

    // For single-item arrays, try to extract a numeric value
    if (item.data.length === 1) {
      const values = Object.values(firstItem);
      const numericValue = values.find((v) => typeof v === "number") as number | undefined;
      if (numericValue !== undefined) {
        return numericValue;
      }
    }

    // Default: return count
    return item.data.length;
  }

  // If data is a single object (shouldn't happen with current insights, but handle it)
  if (typeof item.data === "object" && item.data !== null && !Array.isArray(item.data)) {
    const values = Object.values(item.data);
    const numericValue = values.find((v) => typeof v === "number") as number | undefined;
    if (numericValue !== undefined) {
      return numericValue;
    }
  }

  return "N/A";
};

/**
 * Determines the value type for LabelValue based on insight_id
 */
const getValueType = (insightId: string): "currency" | "amount" | "string" | undefined => {
  if (insightId.includes("balance") || insightId.includes("profit") || insightId === "deals.profit_by_symbol") {
    return "currency";
  }
  if (insightId.includes("count") || insightId.includes("entries")) {
    return "amount";
  }
  return undefined;
};

/**
 * Formats the insight_id as a display label
 */
const formatLabel = (insightId: string): string => {
  // Convert "deals.total_balance" to "Total Balance"
  return insightId
    .split(".")
    .pop()
    ?.split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ") || insightId;
};

export function InsightsGrid({ results, isLoading, error }: InsightsGridProps) {
  if (isLoading) {
    return (
      <div className={styles.container}>
        <div className={styles.loading}>Loading insights...</div>
      </div>
    );
  }

  if (error && (!results || results.length === 0)) {
    return (
      <div className={styles.container}>
        <div className={styles.error}>Error: {error}</div>
      </div>
    );
  }

  if (!results || results.length === 0) {
    return null;
  }

  // Filter to only show successful results (or show errors as well)
  const displayResults = results.filter((r) => r.success);

  if (displayResults.length === 0) {
    return null;
  }

  return (
    <div className={styles.container}>
      <div className={styles.grid}>
        {displayResults.map((item) => {
          const value = extractValue(item);
          const valueType = getValueType(item.insight_id);
          const label = formatLabel(item.insight_id);

          return (
            <LabelValue
              key={item.insight_id}
              label={label}
              value={value}
              valueType={valueType}
              decimals={2}
            />
          );
        })}
      </div>
      {error && (
        <div className={styles.warning}>
          <small>Note: {error}</small>
        </div>
      )}
    </div>
  );
}

