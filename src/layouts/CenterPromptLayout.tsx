import { Component } from "solid-js";
import { cn } from "~/lib/utils";
import { ChildrenProps, ClassProps } from "~/types";

export const CenterPromptLayout: Component<ChildrenProps & ClassProps> = (
  props
) => {
  return (
    <main
      class={cn(
        "flex flex-col h-full items-center justify-center",
        props.class
      )}
    >
      {props.children}
    </main>
  );
};

export type CenterPromptBoxProps = {
  title: string;
} & ChildrenProps;

export const CenterPromptBox: Component<CenterPromptBoxProps> = (props) => {
  return (
    <div class="flex flex-col">
      <h1 class="text-2xl font-bold mb-4">{props.title}</h1>
      {props.children}
    </div>
  );
};
