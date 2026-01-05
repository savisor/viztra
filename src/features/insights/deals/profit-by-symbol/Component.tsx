/**
 * Component for displaying profit_by_symbol insight results
 */

import { useInsight } from "@/shared/insights";
import type { ProfitBySymbolParams, ProfitBySymbolResult } from "./types";
import styles from "./Component.module.css";

interface ProfitBySymbolProps {
  parameters: ProfitBySymbolParams;
  autoFetch?: boolean;
}

export function ProfitBySymbol({ parameters, autoFetch = true }: ProfitBySymbolProps) {
  const { data, isLoading, error, columns, execute } = useInsight<
    ProfitBySymbolParams,
    ProfitBySymbolResult
  >("deals.profit_by_symbol", parameters, { autoFetch });

  if (isLoading) {
    return (
      <div className={styles.container}>
        <div className={styles.loading}>Loading profit by symbol data...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className={styles.container}>
        <div className={styles.error}>Error: {error}</div>
        <button onClick={execute} className={styles.retryButton}>
          Retry
        </button>
      </div>
    );
  }

  if (!data || data.length === 0) {
    return (
      <div className={styles.container}>
        <div className={styles.empty}>No data available</div>
        <button onClick={execute} className={styles.retryButton}>
          Refresh
        </button>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      <div className={styles.table}>
        {/* Header Row */}
        <div className={styles.headerRow}>
          {columns.map((col) => (
            <div key={col} className={styles.headerCell}>
              {col.replace(/_/g, " ").replace(/\b\w/g, (l) => l.toUpperCase())}
            </div>
          ))}
        </div>

        {/* Data Rows */}
        {data.map((row, index) => (
          <div key={index} className={styles.row}>
            {columns.map((col) => {
              const value = row[col as keyof ProfitBySymbolResult];
              const isNumeric = typeof value === "number";
              const isProfit = col === "total_profit" || col === "avg_profit";
              const cellClass = isNumeric
                ? `${styles.cell} ${styles.number} ${
                    isProfit && value < 0 ? styles.negative : ""
                  }`
                : styles.cell;

              return (
                <div key={col} className={cellClass}>
                  {isNumeric
                    ? typeof value === "number"
                      ? value.toLocaleString(undefined, {
                          minimumFractionDigits: 2,
                          maximumFractionDigits: 2,
                        })
                      : value
                    : String(value)}
                </div>
              );
            })}
          </div>
        ))}
      </div>
    </div>
  );
}

