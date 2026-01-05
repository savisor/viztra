import { Container } from "@/shared/ui/Container";
import { useAssetOHLC } from "./hooks";
import { ChartContainer } from "./components/ChartContainer";

export default function ChartScreen() {
  const { data, isLoading, error, symbol, timeframe } = useAssetOHLC(true);

  return (
    <Container
      variant="large"
      style={{ marginInline: "auto" }}
    >
      <ChartContainer
        data={data}
        isLoading={isLoading}
        error={error}
        symbol={symbol}
        timeframe={timeframe}
      />
    </Container>
  );
}

