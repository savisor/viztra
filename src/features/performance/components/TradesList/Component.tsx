import clsx from "clsx";
import type { Deal } from "@/features/deals/types";
import { format } from "date-fns";
import { toZonedTime } from "date-fns-tz";
import styles from "./Component.module.css";

// Format date and time
const formatDateTime = (timestamp: number): string => {
  const date = new Date(timestamp * 1000);
  const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const zonedDate = toZonedTime(date, timeZone);
  return format(zonedDate, "yyyy-MM-dd HH:mm:ss");
};

// Format decimal numbers with proper precision
const formatNumber = (value: number, decimals: number = 2): string => {
  return new Intl.NumberFormat("en-US", {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value);
};

interface TradesListProps {
  deals: Deal[];
  isLoading: boolean;
  error: string | null;
}

export function TradesList({ deals, isLoading, error }: TradesListProps) {
  if (isLoading) {
    return (
      <div className={styles.container}>
        <div className={styles.loading}>Loading trades...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className={styles.container}>
        <div className={styles.error}>Error: {error}</div>
      </div>
    );
  }

  if (deals.length === 0) {
    return (
      <div className={styles.container}>
        <div className={styles.empty}>No trades found</div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      <div className={styles.table}>
        {/* Header Row */}
        <div className={styles.headerRow}>
          <div className={clsx(styles.cell, styles.text)}>Position ID</div>
          <div className={clsx(styles.cell, styles.text)}>Order</div>
          <div className={clsx(styles.cell, styles.time)}>Time</div>
          <div className={clsx(styles.cell, styles.text)}>Symbol</div>
          <div className={clsx(styles.cell, styles.number)}>Volume</div>
          <div className={clsx(styles.cell, styles.number)}>Price</div>
          <div className={clsx(styles.cell, styles.number)}>Commission</div>
          <div className={clsx(styles.cell, styles.number)}>Swap</div>
          <div className={clsx(styles.cell, styles.number)}>Profit</div>
          <div className={clsx(styles.cell, styles.number)}>Fee</div>
          <div className={clsx(styles.cell, styles.text)}>Comment</div>
        </div>

        {/* Data Rows */}
        {deals.map((deal) => (
          <div key={deal.ticket} className={styles.row}>
            <div className={clsx(styles.cell, styles.text)}>{deal.position_id}</div>
            <div className={clsx(styles.cell, styles.text)}>{deal.order}</div>
            <div className={clsx(styles.cell, styles.time)}>{formatDateTime(deal.time)}</div>
            <div className={clsx(styles.cell, styles.text)}>{deal.symbol}</div>
            <div className={clsx(styles.cell, styles.number)}>{formatNumber(deal.volume, 2)}</div>
            <div className={clsx(styles.cell, styles.number)}>{formatNumber(deal.price, 5)}</div>
            <div className={clsx(styles.cell, styles.number)}>{formatNumber(deal.commission, 2)}</div>
            <div className={clsx(styles.cell, styles.number)}>{formatNumber(deal.swap, 2)}</div>
            <div className={clsx(styles.cell, styles.number, deal.profit >= 0 ? styles.profit : styles.loss)}>
              {formatNumber(deal.profit, 2)}
            </div>
            <div className={clsx(styles.cell, styles.number)}>{formatNumber(deal.fee, 2)}</div>
            <div className={clsx(styles.cell, styles.text)}>{deal.comment}</div>
          </div>
        ))}
      </div>
    </div>
  );
}

