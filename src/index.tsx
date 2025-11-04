/* @refresh reload */
import { render } from "solid-js/web";
import "./index.css";
import {
  createEffect,
  createSignal,
  Match,
  Switch,
  type Component,
} from "solid-js";
import { Route, Router } from "@solidjs/router";
import { LoginPage } from "./pages/login/LoginPage";
import { NotFoundPage } from "./pages/globs/NotFound";
import { DashboardPage } from "./pages/dashboard/DashboardPage";
import { useAliyunClientStatus } from "./lib/auth";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { CenterPromptLayout } from "./layouts/CenterPromptLayout";

const RootContainer: Component = () => {
  /**
   * todo: add logic about hasValidClient.
   * i.e., when hasValidClient is ready, redirect to the target page.
   */
  const { hasValidClient, hasClient } = useAliyunClientStatus();
  const [show, setShow] = createSignal(false);

  createEffect(() => {
    console.debug(
      `debug client status (has client), loading: ${
        hasClient.loading
      }, value: ${hasClient()}`
    );

    if (!show() && !hasClient.loading) {
      setShow(true);

      const currentWindow = getCurrentWindow();
      currentWindow
        .show()
        .then(() => console.info("Window is shown."))
        .catch((e) => console.error(e));
    }
  });

  return (
    <Router>
      <Route path={"/login"} component={LoginPage} />
      <Route
        path={"/"}
        component={() => (
          <Switch>
            <Match when={!hasClient.loading && hasClient()}>
              <DashboardPage></DashboardPage>
            </Match>
            <Match when={hasClient.loading}>
              <CenterPromptLayout class="bg-blue-800"></CenterPromptLayout>
            </Match>
            <Match when={true}>
              <LoginPage></LoginPage>
            </Match>
          </Switch>
        )}
      />
      <Route path={"*404"} component={NotFoundPage} />
    </Router>
  );
};

render(() => <RootContainer />, document.getElementById("root") as HTMLElement);
