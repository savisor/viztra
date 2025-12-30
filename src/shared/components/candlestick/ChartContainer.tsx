import { Chart, Loading, Error, WaterMark } from "./index";
import type { OHLCData } from "@/types";
import styles from "./ChartContainer.module.css";

interface ChartContainerProps {
  data?: OHLCData[];
  isLoading: boolean;
  error: string | null;
  symbol: string;
  timeframe: string;
}

export function ChartContainer({
  data,
  isLoading,
  error,
  symbol,
  timeframe,
}: ChartContainerProps) {
  return (
    <div className={styles.container}>
      <Chart data={data} />
      <WaterMark symbol={symbol} timeframe={timeframe} />
      {isLoading && <Loading />}
      {error && <Error error={error} />}
    </div>
  );
}

