interface TimeframeSelectorProps {
  selectedTimeframe: string;
  onTimeframeSelect: (timeframe: string) => void;
}

const TIMEFRAMES = ["5M", "15M", "30M", "1H", "4H", "1D", "1W", "1M"] as const;

/**
 * Timeframe selector component - plain JSX, no styles
 */
export function TimeframeSelector({
  selectedTimeframe,
  onTimeframeSelect,
}: TimeframeSelectorProps) {
  return (
    <div>
      <h3>Timeframe</h3>
      <div>
        {TIMEFRAMES.map((tf) => (
          <button
            key={tf}
            type="button"
            onClick={() => onTimeframeSelect(tf)}
          >
            {selectedTimeframe === tf ? "✓ " : ""}
            {tf}
          </button>
        ))}
      </div>
    </div>
  );
}
