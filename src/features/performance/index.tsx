import { Container } from "@/shared/ui/Container";
import { DealImportModal } from "../deals";
import { Header, TradesList, InsightsGrid } from "./components";
import { useInsight } from "@/shared/insights";
import { useBatchInsights } from "@/shared/insights";
import type { TradeEntriesParams } from "@/features/insights/deals/trade-entries/types";
import type { Deal } from "@/features/deals/types";
import { useState } from "react";
import { useKeyboardShortcut } from "@/shared/hooks";

export default function PerformanceScreen() {
  const [dealImportModalOpen, setDealImportModalOpen] = useState(false);
  const { data: deals, isLoading, error } = useInsight<TradeEntriesParams, Deal>(
    "deals.trade_entries",
    { account_number: "5043757397" },
    { autoFetch: true }
  );

  // Batch insights execution - all existing insights
  const {
    data: batchResults,
    isLoading: isBatchLoading,
    error: batchError,
  } = useBatchInsights(
    [
      {
        insight_id: "deals.total_balance",
        parameters: { account_number: "5043757397" },
      },
      {
        insight_id: "deals.trade_entries",
        parameters: { account_number: "5043757397" },
      },
      {
        insight_id: "deals.balance_entries",
        parameters: { account_number: "5043757397" },
      },
      {
        insight_id: "deals.all_entries",
        parameters: { account_number: "5043757397" },
      },
      {
        insight_id: "deals.trade_entries_with_balance",
        parameters: { account_number: "5043757397" },
      },
      {
        insight_id: "deals.profit_by_symbol",
        parameters: { account_number: "5043757397" },
      },
    ],
    { autoFetch: true }
  );

  // Keyboard shortcut for Ctrl+D - opens DealImportModal
  useKeyboardShortcut(
    "d",
    () => {
      setDealImportModalOpen((prev) => !prev);
    },
    { ctrlKey: true }
  );

  return (
    <Container variant="large" style={{ marginInline: "auto" }}>
      <DealImportModal
        open={dealImportModalOpen}
        onClose={() => setDealImportModalOpen(false)}
      />
      <Container variant="large" bordered="none" padding="0">
        <Header onImportClick={() => setDealImportModalOpen(true)} />
        <InsightsGrid
          results={batchResults || []}
          isLoading={isBatchLoading}
          error={batchError}
        />
        <TradesList deals={deals || []} isLoading={isLoading} error={error} />
      </Container>
    </Container>
  );
}
