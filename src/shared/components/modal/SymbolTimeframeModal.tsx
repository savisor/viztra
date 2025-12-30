import { Modal } from "./Modal";
import { SymbolSelector } from "./SymbolSelector";
import { TimeframeSelector } from "./TimeframeSelector";
import { useSymbolTimeframe } from "@/shared/hooks";

interface SymbolTimeframeModalProps {
  open: boolean;
  onClose: () => void;
}

/**
 * Modal for selecting symbol and timeframe
 * Connects to SymbolTimeframeProvider and triggers refetch on change
 */
export function SymbolTimeframeModal({
  open,
  onClose,
}: SymbolTimeframeModalProps) {
  const { symbol, timeframe, setSymbol, setTimeframe } = useSymbolTimeframe();

  const handleSymbolSelect = (selectedSymbol: string) => {
    setSymbol(selectedSymbol);
  };

  const handleTimeframeSelect = (selectedTimeframe: string) => {
    setTimeframe(selectedTimeframe);
  };

  return (
    <Modal open={open} onClose={onClose} title="Select Symbol and Timeframe">
      <TimeframeSelector
        selectedTimeframe={timeframe}
        onTimeframeSelect={handleTimeframeSelect}
      />
      <SymbolSelector
        selectedSymbol={symbol}
        onSymbolSelect={handleSymbolSelect}
      />
    </Modal>
  );
}

