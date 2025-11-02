import { createEffect, createMemo, createSignal, Resource } from "solid-js";
import { createResource } from "solid-js";
import { debounce } from "@solid-primitives/scheduled";
import { AccessKeyCredentials, commands } from "~/binding";
export type { AccessKeyCredentials } from "~/binding";

export enum AccessKeyUsability {
  Waiting,
  Usable,
  Unusable,
}

export function ensureAccessKeyUsable(
  credentials: AccessKeyCredentials
): Resource<AccessKeyUsability> {
  const [creds, setCreds] = createSignal(credentials as AccessKeyCredentials);

  const updateCredentials = debounce(setCreds, 300);

  createEffect(() => {
    updateCredentials({
      access_key_id: credentials.access_key_id,
      access_key_secret: credentials.access_key_secret,
    });
  });

  const [usability] = createResource(
    creds,
    async (creds): Promise<AccessKeyUsability> => {
      console.debug("Validating access key credentials:", creds);

      if (
        creds.access_key_id.length < 10 ||
        creds.access_key_secret.length < 10
      ) {
        return AccessKeyUsability.Waiting;
      }

      let result: AccessKeyUsability = AccessKeyUsability.Unusable;
      try {
        const resp = await commands.validateAccessKeyCredentials(creds);
        if (resp.status == "ok") {
          result = AccessKeyUsability.Usable;
        } else throw resp;
      } catch (error) {
        console.error("Error fetching STS caller identity:");
        console.error(error);
        result = AccessKeyUsability.Unusable;
      }
      return result;
    }
  );

  return usability;
}
