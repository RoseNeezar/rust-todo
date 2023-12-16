import { QueryClient } from "@tanstack/react-query";
import { FetchTransport, createClient } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";

import type { Procedures } from "./api"; // These are generated by rspc in Rust for you.

const client = createClient<Procedures>({
  transport: new FetchTransport("http://localhost:8000/rspc"),
});

const queryClient = new QueryClient();
const rspc = createReactQueryHooks<Procedures>();

export { rspc, client, queryClient };
