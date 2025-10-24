import { Component } from "solid-js";
import { CenterPromptBox, CenterPromptLayout } from "~/layouts/CenterPromptLayout";

export const LoginPage: Component = () => {
  return <CenterPromptLayout>
    <CenterPromptBox title="登陆阿里云">
      <div></div>
    </CenterPromptBox>
  </CenterPromptLayout>
}