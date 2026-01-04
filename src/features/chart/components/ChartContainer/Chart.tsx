import { useRef, useEffect } from "react";
import {
  CandlestickSeries,
  createChart,
  type IChartApi,
  type ISeriesApi,
  type UTCTimestamp,
} from "lightweight-charts";
import type { OHLCData } from "../../types";
import styles from "./Chart.module.css";

interface ChartProps {
  data?: OHLCData[];
}

// Convert OHLCData to lightweight-charts format
function convertToCandlestickData(data: OHLCData[]) {
  return data.map((item) => ({
    time: item.time as UTCTimestamp,
    open: item.open,
    high: item.high,
    low: item.low,
    close: item.close,
  }));
}

export function Chart({ data }: ChartProps) {
  const chartContainerRef = useRef<HTMLDivElement>(null);
  const chartRef = useRef<IChartApi | null>(null);
  const seriesRef = useRef<ISeriesApi<"Candlestick"> | null>(null);

  useEffect(() => {
    if (!chartContainerRef.current) return;

    // Create chart
    const chart = createChart(chartContainerRef.current, {
      autoSize: true,
      layout: {
        background: { color: "#000000" },
        textColor: "#d1d5db",
        attributionLogo: false,
      },
      leftPriceScale: { visible: false },
      rightPriceScale: { visible: true },
      grid: {
        vertLines: { visible: false },
        horzLines: { visible: false },
      },
    });

    chartRef.current = chart;

    // Add candlestick series (v5 API)
    const candlestickSeries = chart.addSeries(CandlestickSeries, {
      upColor: "#26a69a",
      downColor: "#ef5350",
      borderVisible: false,
      wickUpColor: "#26a69a",
      wickDownColor: "#ef5350",
    });

    seriesRef.current = candlestickSeries;

    // No mock data: start empty and wait for API data.
    candlestickSeries.setData([]);

    // Fit content (empty -> no-op)
    chart.timeScale().fitContent();

    return () => {
      chart.remove();
      chartRef.current = null;
      seriesRef.current = null;
    };
  }, []);

  // Update series data when API data arrives
  useEffect(() => {
    if (seriesRef.current && data) {
      const candlestickData = convertToCandlestickData(data);
      seriesRef.current.setData(candlestickData);
      if (candlestickData.length > 0) {
        chartRef.current?.timeScale().fitContent();
      }
    }
  }, [data]);

  return (
    <div
      ref={chartContainerRef}
      className={styles.chartContainer}
    />
  );
}
