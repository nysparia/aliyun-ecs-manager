import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  Show,
} from "solid-js";
import { CenterPromptLayout } from "~/layouts/CenterPromptLayout";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "~/components/ui/card";
import {
  TextField,
  TextFieldInput,
  TextFieldLabel,
} from "~/components/ui/text-field";
import { Button } from "~/components/ui/button";
import { createStore } from "solid-js/store";
import { createInputUpdater } from "~/lib/utils/form";
import {
  AccessKeyCredentials,
  AccessKeyUsability,
  ensureAccessKeyUsable,
} from "./utils";
import { cn } from "~/lib/utils";
import { ChildrenProps, ClassProps } from "~/types";

export const LoginPage: Component = () => {
  const [loginFormData, setLoginFormData] = createStore<AccessKeyCredentials>({
    access_key_id: "",
    access_key_secret: "",
  });

  const [errorText, setErrorText] = createSignal("");
  const clearErrorText = () => setErrorText("");

  const createFormDataUpdateFn =
    (field: keyof typeof loginFormData) => (value: string) => {
      setLoginFormData(field, value);
      clearErrorText();
      // console.debug("Updated login form data:", field, value);
      // console.debug(loginFormData);
    };

  const createFormDataUpdater = (field: keyof typeof loginFormData) =>
    createInputUpdater(createFormDataUpdateFn(field));

  const { usability, fulfillCredentials } =
    ensureAccessKeyUsable(loginFormData);

  const usabilityConfigs = {
    [AccessKeyUsability.Unusable]: { text: "凭证无效", color: "text-red-800" },
    [AccessKeyUsability.Usable]: { text: "凭证有效", color: "text-green-800" },
    [AccessKeyUsability.Waiting]: { text: "等待输入", color: "text-gray-400" },
  };

  const currentUsabilityConfig = () => {
    return usabilityConfigs[usability() ?? AccessKeyUsability.Waiting];
  };

  const [hintShakeTrigger, setHintShakeTrigger] = createSignal(0);
  const [submitDisabled, setSubmitDisabled] = createSignal(false);

  const onSubmit = async () => {
    setSubmitDisabled(true);
    try {
      const usability = await fulfillCredentials();
      // console.debug(usability);
      if (usability != AccessKeyUsability.Usable) {
        // console.debug("Login attempt with unusable credentials.", usability);
        setHintShakeTrigger(hintShakeTrigger() + 1);
      }
      // setErrorText(`内部错误`);
    } catch (err) {
      setErrorText(`内部错误：${err}`);
    } finally {
      setSubmitDisabled(false);
    }
  };

  return (
    <CenterPromptLayout class="bg-amber-300">
      <Card>
        <CardHeader>
          <CardTitle>登陆阿里云</CardTitle>
          <CardDescription>
            请使用您的阿里云RAM账号access key完成登陆
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form class="flex flex-col gap-4">
            <TextField>
              <TextFieldLabel>Access Key ID</TextFieldLabel>
              <TextFieldInput
                id={"aliyun-access-key-id"}
                name={"aliyun-access-key-id"}
                type="text"
                // autocomplete="username"
                required
                value={loginFormData.access_key_id}
                onInput={createFormDataUpdater("access_key_id")}
              />
            </TextField>
            <TextField>
              <TextFieldLabel>Access Key Secret</TextFieldLabel>
              <TextFieldInput
                id={"aliyun-access-key-secret"}
                name={"aliyun-access-key-secret"}
                type="password"
                // autocomplete="current-password"
                required
                value={loginFormData.access_key_secret}
                onInput={createFormDataUpdater("access_key_secret")}
              />
            </TextField>
          </form>
        </CardContent>
        <CardFooter class="flex justify-between">
          <ShakeSpan
            trigger={hintShakeTrigger}
            class={currentUsabilityConfig().color}
          >
            {currentUsabilityConfig().text}
          </ShakeSpan>
          <div class="flex flex-1 flex-row-reverse items-center gap-4">
            <Button onclick={onSubmit} disabled={submitDisabled()}>
              登陆
            </Button>
            <Show when={errorText()}>
              <span class="text-sm text-red-800">{errorText()}</span>
            </Show>
          </div>
        </CardFooter>
      </Card>
    </CenterPromptLayout>
  );
};

const ShakeSpan: Component<
  ChildrenProps & ClassProps & { trigger: Accessor<number> }
> = (props) => {
  const [isShaking, setIsShaking] = createSignal(false);

  createEffect(() => {
    const triggerValue = props.trigger();
    console.debug(triggerValue);
    if (triggerValue <= 0) return;
    setIsShaking(false);

    setTimeout(() => {
      setIsShaking(true);
    }, 10);
  });

  return (
    <span class={cn(`text-sm`, isShaking() && "animate-shake", props.class)}>
      {props.children}
    </span>
  );
};
