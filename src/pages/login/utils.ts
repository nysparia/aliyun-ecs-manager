import { createEffect, createSignal, Resource } from "solid-js";
import { createResource } from "solid-js";
import { debounce } from "@solid-primitives/scheduled";
import { AccessKeyCredentials, commands } from "~/binding";
export type { AccessKeyCredentials } from "~/binding";

export enum AccessKeyUsability {
  Waiting,
  Usable,
  Unusable,
}

export function ensureAccessKeyUsable(credentials: AccessKeyCredentials): {
  usability: Resource<AccessKeyUsability>;
  refetch: () => Promise<AccessKeyUsability>;
  fulfillCredentials: () => Promise<
    AccessKeyUsability.Usable | AccessKeyUsability.Unusable
  >;
} {
  const [creds, setCreds] = createSignal(credentials as AccessKeyCredentials);

  const updateCredentials = debounce(setCreds, 300);

  createEffect(() => {
    updateCredentials({
      access_key_id: credentials.access_key_id,
      access_key_secret: credentials.access_key_secret,
    });
  });

  const [usability, { refetch, mutate }] = createResource(
    creds,
    async (creds): Promise<AccessKeyUsability> => {
      console.debug("Validating access key credentials:", creds);

      if (
        creds.access_key_id.length < 10 ||
        creds.access_key_secret.length < 10
      ) {
        return AccessKeyUsability.Waiting;
      }

      let result: AccessKeyUsability;
      try {
        const resp = await commands.validateAccessKeyCredentials(creds);
        if (resp.status == "ok") {
          result = AccessKeyUsability.Usable;
        } else if (
          resp.status == "error" &&
          resp.error.type == "Specific" &&
          resp.error.error.type === "AKNotValid"
        ) {
          result = AccessKeyUsability.Unusable;
        } else {
          throw resp.error;
        }
      } catch (error) {
        console.error("Error fetching STS caller identity:");
        console.error(error);
        result = AccessKeyUsability.Unusable;
      }
      return result;
    }
  );

  return {
    usability,
    refetch: async () => {
      return (await refetch()) as AccessKeyUsability;
    },
    fulfillCredentials: async () => {
      const creds: AccessKeyCredentials = {
        access_key_id: credentials.access_key_id,
        access_key_secret: credentials.access_key_secret,
      };
      const r = await commands.fulfillAccessKeyCredentials(creds);
      if (r.status === "ok") {
        return mutate(AccessKeyUsability.Usable);
      } else {
        const error = r.error;
        if (error.type == "Specific" && error.error.type === "AKNotValid") {
          return mutate(AccessKeyUsability.Unusable);
        } else {
          throw r.error;
        }
      }
    },
  };
}
