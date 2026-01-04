import { useState } from "react";
import { AppProvider } from "./providers";
import { useAssetOHLC } from "./hooks";
import { ChartContainer } from "./components/ChartContainer";
import { SymbolTimeframeModal } from "./components/SymbolTimeframeModal";
import { useKeyboardShortcut } from "@/shared/hooks";

function ChartComponent() {
  const { data, isLoading, error, symbol, timeframe } = useAssetOHLC(true);

  return (
    <ChartContainer
      data={data}
      isLoading={isLoading}
      error={error}
      symbol={symbol}
      timeframe={timeframe}
    />
  );
}

export default function ChartScreen() {
  const [symbolTimeframeModalOpen, setSymbolTimeframeModalOpen] = useState(false);


  // Keyboard shortcut for Cmd+A / Ctrl+A
  useKeyboardShortcut(
    "a",
    () => {
      setSymbolTimeframeModalOpen((prev) => !prev);
    },
    { metaKey: true, ctrlKey: true }
  );

  return (
    <AppProvider>
      <SymbolTimeframeModal
        open={symbolTimeframeModalOpen}
        onClose={() => setSymbolTimeframeModalOpen(false)}
      />
      <ChartComponent />
    </AppProvider>
  );
}
