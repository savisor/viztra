import clsx from "clsx";
import styles from "./Component.module.css";
import {  type DateRangePreset } from "../../helpers/dateRange";

export interface DateRangePickerProps {
  value: DateRangePreset;
  onChange: (preset: DateRangePreset) => void;
  className?: string;
}

const PRESET_OPTIONS: Array<{ value: DateRangePreset; label: string }> = [
  { value: "today", label: "Today" },
  { value: "yesterday", label: "Yesterday" },
  { value: "lastWeek", label: "Last Week" },
  { value: "lastMonth", label: "Last Month" },
  { value: "lastYear", label: "Last Year" },
  { value: "allTime", label: "All Time" },
];

export const DateRangePicker = ({
  value,
  onChange,
  className,
}: DateRangePickerProps) => {
  return (
    <div className={clsx(styles.container, className)}>
      <select
        className={styles.select}
        value={value}
        onChange={(e) => onChange(e.target.value as DateRangePreset)}
      >
        {PRESET_OPTIONS.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </div>
  );
};

DateRangePicker.displayName = "DateRangePicker";

