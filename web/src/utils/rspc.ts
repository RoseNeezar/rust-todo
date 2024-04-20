import { QueryClient } from "@tanstack/react-query";
import { FetchTransport, createClient } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";

import type { Procedures } from "./api"; // These are generated by rspc in Rust for you.
import { supabase } from "../main";

const request = async (input: RequestInfo | URL, init?: RequestInit) => {
  const token = (await supabase.auth.getSession()).data.session?.access_token;
  const resp = await fetch(input, {
    ...init,
    credentials: "include",
    headers: {
      ...init?.headers,
      authorization: `Bearer ${token}`,
    },
  });
  return resp;
};

const client = createClient<Procedures>({
  transport: new FetchTransport(import.meta.env.SERVER_URL, request),
});

const queryClient = new QueryClient();
const rspc = createReactQueryHooks<Procedures>();

export { rspc, client, queryClient };
