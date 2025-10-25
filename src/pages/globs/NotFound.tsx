import { useLocation, useNavigate } from "@solidjs/router";
import type { Component } from "solid-js";
import { CenterPromptLayout } from "~/layouts/CenterPromptLayout";

const GITHUB_URL = "https://github.com/nysparia";

export const NotFoundPage: Component = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const navigateBack = () => {
    console.debug("Navigating back to previous page.");
    navigate(-1);
  };

  return (
    <CenterPromptLayout>
      <div class="mb-8 rounded-md overflow-hidden">
        <a href={GITHUB_URL} target="_blank">
          <img src="/neko.jpg" class="h-[200px] w-[200px]" />
        </a>
      </div>
      <h2 class="text-2xl">404 Page Not Found.</h2>
      <hr class="my-4" />
      <div class="text-sm/6 text-stone-400 flex flex-col items-center">
        <p>The page you are looking for does not exist.</p>
        <p>Current URL: {location.pathname}</p>
        <a
          class="underline hover:text-black transition-colors duration-200"
          onclick={navigateBack}
          href=""
        >
          Return to last location.
        </a>
      </div>
    </CenterPromptLayout>
  );
};
