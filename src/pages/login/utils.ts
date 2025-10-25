import { Config } from "@alicloud/credentials";
import Sts from "@alicloud/sts20150401";
import { Resource } from "solid-js";
import { createResource } from "solid-js";

export interface AccessKeyCredentials {
  accessKeyId: string;
  accessKeySecret: string;
}

export enum AccessKeyUsability {
  Waiting,
  Usable,
  Unusable,
}

const HANGZHOU_ENDPOINT = "sts.cn-hangzhou.aliyuncs.com";

export function ensureAccessKeyUsable(
  credentials: AccessKeyCredentials
): Resource<AccessKeyUsability> {
  const [usability] = createResource(
    credentials,
    async (credentials): Promise<AccessKeyUsability> => {
      if (
        credentials.accessKeyId.length < 10 ||
        credentials.accessKeySecret.length < 10
      ) {
        return AccessKeyUsability.Waiting;
      }

      const credentialConfig = new Config({
        accessKeyId: credentials.accessKeyId,
        accessKeySecret: credentials.accessKeySecret,
      });
      const stsClient = new Sts(credentialConfig);

      let result: AccessKeyUsability = AccessKeyUsability.Unusable;
      try {
        const resp = await stsClient.getCallerIdentity();
        console.log(typeof resp === "string" ? resp : JSON.stringify(resp));
        result = AccessKeyUsability.Usable;
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
