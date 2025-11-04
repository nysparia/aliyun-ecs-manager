import { createResource } from "solid-js";
import { commands } from "~/binding";

const [hasValidClient, { refetch: revalidateClient }] = createResource(
  async (): Promise<boolean> => {
    try {
      const result = await commands.hasValidAliyunClient();
      if (result.status == "error") {
        throw result.error;
      }
      return result.data;
    } catch (error) {
      console.error(error);
      return false;
    }
  }
);

const [hasClient, { refetch }] = createResource(async (): Promise<boolean> => {
  try {
    const result = await commands.hasAliyunClient();
    if (result.status == "ok") {
      return result.data;
    } else {
      throw result.error;
    }
  } catch (error) {
    console.error(error);
    return false;
  }
});

export function useAliyunClientStatus() {
  return {
    hasClient,
    hasValidClient,
    revalidate: async () => {
      await revalidateClient();
      await refetch();
    },
  };
}