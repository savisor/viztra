import clsx from "clsx";
import React from "react";
import styles from "./Component.module.css";
import {
  Gap,
  Margin,
  Padding,
  Direction,
  InlineAlign,
  BlockAlign,
  Bordered,
} from "../types";

type StackComponent =
  | "div"
  | "nav"
  | "section"
  | "article"
  | "header"
  | "footer"
  | "aside"
  | "main"
  | "form"
  | "ul"
  | "ol"
  | "li";

export interface StackProps extends React.HTMLAttributes<HTMLElement> {
  direction?: Direction;
  gap?: Gap;
  padding?: Padding;
  paddingInline?: Padding;
  paddingBlock?: Padding;
  margin?: Margin;
  inlineAlign?: InlineAlign;
  blockAlign?: BlockAlign;
  bordered?: Bordered;
  component?: StackComponent;
}

export const Stack = ({
  className,
  direction,
  gap,
  padding,
  paddingInline,
  paddingBlock,
  margin,
  inlineAlign,
  blockAlign,
  bordered,
  component = "div",
  ...props
}: StackProps) => {
  return React.createElement(
    component,
    {
      className: clsx(styles.stack, className),
      "data-direction": direction,
      "data-gap": gap,
      "data-padding": padding,
      "data-margin": margin,
      "data-inline-align": inlineAlign,
      "data-block-align": blockAlign,
      "data-bordered": bordered,
      "data-padding-inline": paddingInline,
      "data-padding-block": paddingBlock,
      ...props,
    },
    props.children
  );
};
