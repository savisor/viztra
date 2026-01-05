import clsx from "clsx";
import styles from "./Component.module.css";
import type { Padding, Shape } from "@/shared/ui/types";

export type ValueType =
  | "string"
  | "boolean"
  | "amount"
  | "percentage"
  | "currency"
  | "positive"
  | "negative";

export interface LabelValueProps {
  label: string;
  value: React.ReactNode;
  valueType?: ValueType;
  color?: string;
  currencyCode?: string;
  decimals?: number;
  padding?: Padding;
  shape?: Shape;
  className?: string;
  style?: React.CSSProperties;
}

const formatNumber = (value: number, decimals: number = 2): string => {
  return new Intl.NumberFormat("en-US", {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value);
};

const formatCurrency = (
  value: number,
  currencyCode: string = "USD",
  decimals: number = 2
): string => {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: currencyCode,
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value);
};

const formatPercentage = (value: number, decimals: number = 2): string => {
  return `${formatNumber(value, decimals)}%`;
};

const renderValue = (
  value: React.ReactNode,
  valueType?: ValueType,
  color?: string,
  currencyCode?: string,
  decimals?: number
): React.ReactNode => {
  // If valueType is not provided, render as-is
  if (!valueType) {
    return <span style={color ? { color } : undefined}>{value}</span>;
  }

  // Handle boolean values with badge
  if (valueType === "boolean") {
    const boolValue =
      typeof value === "boolean"
        ? value
        : typeof value === "string"
          ? value.toLowerCase() === "true"
          : Boolean(value);
    const badgeClass = boolValue ? styles.badgeSuccess : styles.badgeError;
    const badgeText = boolValue ? "Yes" : "No";
    
    return (
      <span
        className={clsx(styles.badge, badgeClass)}
        style={color ? { backgroundColor: color, color: "white" } : undefined}
      >
        {badgeText}
      </span>
    );
  }

  // Handle numeric values
  if (typeof value === "number") {
    let formattedValue: string;
    let valueColor: string | undefined = color;

    switch (valueType) {
      case "currency":
        formattedValue = formatCurrency(value, currencyCode, decimals);
        break;
      case "percentage":
        formattedValue = formatPercentage(value, decimals);
        break;
      case "positive":
        formattedValue = formatNumber(value, decimals);
        valueColor = color || "var(--color-success)";
        break;
      case "negative":
        formattedValue = formatNumber(value, decimals);
        valueColor = color || "var(--color-error)";
        break;
      case "amount":
        formattedValue = formatNumber(value, decimals);
        break;
      default:
        formattedValue = formatNumber(value, decimals);
    }

    return <span style={valueColor ? { color: valueColor } : undefined}>{formattedValue}</span>;
  }

  // Handle string values that might need formatting
  if (typeof value === "string" && valueType !== "string") {
    const numValue = parseFloat(value);
    if (!isNaN(numValue)) {
      return renderValue(numValue, valueType, color, currencyCode, decimals);
    }
  }

  // Default: render as-is with optional color
  return <span style={color ? { color } : undefined}>{value}</span>;
};

export const LabelValue = ({
  label,
  value,
  valueType,
  color,
  currencyCode = "USD",
  decimals = 2,
  padding = "4",
  shape = "md",
  className,
  style,
}: LabelValueProps) => {
  const renderedValue = renderValue(value, valueType, color, currencyCode, decimals);

  return (
    <div
      className={clsx(styles.labelValue, className)}
      data-padding={padding}
      data-shape={shape}
      style={style}
    >
      <div className={styles.label}>{label}</div>
      <div className={styles.value}>{renderedValue}</div>
    </div>
  );
};

LabelValue.displayName = "LabelValue";

