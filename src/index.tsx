/* @refresh reload */
import { render } from "solid-js/web";
import "./index.css";
import type { Component } from "solid-js";
import { Route, Router } from "@solidjs/router";
import { LoginPage } from "./pages/login/LoginPage";

const RootContainer: Component = () => (
  <Router>
    <Route path={"/login"} component={LoginPage} />
  </Router>
);

render(() => <RootContainer />, document.getElementById("root") as HTMLElement);
