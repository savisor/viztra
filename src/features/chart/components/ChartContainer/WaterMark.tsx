import styles from "./WaterMark.module.css";

interface WaterMarkProps {
  symbol: string;
  timeframe: string;
}

export function WaterMark({ symbol, timeframe }: WaterMarkProps) {
  return (
    <div className={styles.watermark}>
      <div className={styles.content}>
        <div>{symbol}</div>
        <div>{timeframe}</div>
      </div>
    </div>
  );
}
