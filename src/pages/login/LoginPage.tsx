import { Component } from "solid-js";
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

export const LoginPage: Component = () => {
  const [loginFormData, setLoginFormData] = createStore<AccessKeyCredentials>({
    access_key_id: "",
    access_key_secret: "",
  });

  const createFormDataUpdateFn =
    (field: keyof typeof loginFormData) => (value: string) =>
    {
      setLoginFormData(field, value);
      // console.debug("Updated login form data:", field, value);
      // console.debug(loginFormData);
    }

  const createFormDataUpdater = (field: keyof typeof loginFormData) =>
    createInputUpdater(createFormDataUpdateFn(field));

  const usability = ensureAccessKeyUsable(loginFormData);

  const usabilityConfigs = {
    [AccessKeyUsability.Unusable]: { text: "凭证无效", color: "red" },
    [AccessKeyUsability.Usable]: { text: "凭证有效", color: "green" },
    [AccessKeyUsability.Waiting]: { text: "等待输入", color: "grey" },
  };

  const currentUsabilityConfig = () => {
    return usabilityConfigs[usability() ?? AccessKeyUsability.Waiting];
  };

  return (
    <CenterPromptLayout>
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
                autocomplete="username"
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
                autocomplete="current-password"
                required
                value={loginFormData.access_key_secret}
                onInput={createFormDataUpdater("access_key_secret")}
              />
            </TextField>
          </form>
        </CardContent>
        <CardFooter class="flex justify-between">
          <span>{currentUsabilityConfig().text}</span>
          <Button>登陆</Button>
        </CardFooter>
      </Card>
    </CenterPromptLayout>
  );
};
