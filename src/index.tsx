/* @refresh reload */
import { render } from "solid-js/web";
import "./index.css";
import type { Component } from "solid-js";
import { Route, Router } from "@solidjs/router";
import { LoginPage } from "./pages/login/LoginPage";
import { NotFoundPage } from "./pages/globs/NotFound";

const RootContainer: Component = () => (
  <Router>
    <Route path={"/"} component={LoginPage} />
    <Route path={"*404"} component={NotFoundPage} />
  </Router>
);

render(() => <RootContainer />, document.getElementById("root") as HTMLElement);
