import { Component } from "solid-js";
import { ChildrenProps } from "~/types";

export const CenterPromptLayout: Component<ChildrenProps> = (props) => {
  return <main class="flex flex-col h-full items-center justify-center">{props.children}</main>;
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
