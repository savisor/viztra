import clsx from "clsx";
import React from "react";
import styles from "./Component.module.css";

export interface ActionListItem {
  content: React.ReactNode;
}

export interface ActionListProps extends React.HTMLAttributes<HTMLUListElement> {
  items: ActionListItem[];
  className?: string;
}

export const ActionList = ({
  items,
  className,
  ...props
}: ActionListProps) => {
  return (
    <ul className={clsx(styles["action-list"], className)} {...props}>
      {items.map((item, index) => (
        <li key={index} className={styles["action-list-item"]}>
          <span className={styles["action-list-bullet"]}>[*]</span>
          {item.content}
        </li>
      ))}
    </ul>
  );
};

