import clsx from "clsx";
import styles from "./Component.module.css";
import type { Padding, Shape } from "@/shared/ui/types";

export interface CardProps {
  children: React.ReactNode;
  padding?: Padding;
  shape?: Shape;
  className?: string;
  style?: React.CSSProperties;
}

export const Card = ({
  children,
  padding = "4",
  shape = "md",
  className,
  style,
}: CardProps) => {
  return (
    <div
      className={clsx(styles.card, className)}
      data-padding={padding}
      data-shape={shape}
      style={style}
    >
      {children}
    </div>
  );
};

Card.displayName = "Card";
