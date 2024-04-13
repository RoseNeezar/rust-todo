import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { rspc, client, queryClient } from "./utils/rspc";
import { QueryClientProvider } from "@tanstack/react-query";
import "./styles/globals.css";

import { useState, useEffect } from "react";
import { createClient } from "@supabase/supabase-js";
import { Auth } from "@supabase/auth-ui-react";
import { ThemeSupa } from "@supabase/auth-ui-shared";

export const supabase = createClient(
  import.meta.env.VITE_SUPABASE_URL,
  import.meta.env.VITE_SUPABASE_KEY
);

function Layout() {
  const [session, setSession] = useState<any>(null);

  useEffect(() => {
    supabase.auth.getSession().then(({ data: { session } }) => {
      setSession(session);
    });

    const {
      data: { subscription },
    } = supabase.auth.onAuthStateChange((_event, session) => {
      setSession(session);
    });

    return () => subscription.unsubscribe();
  }, []);

  if (!session) {
    return <Auth supabaseClient={supabase} appearance={{ theme: ThemeSupa }} />;
  } else {
    return <App />;
  }
}

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <rspc.Provider client={client} queryClient={queryClient}>
      <QueryClientProvider client={queryClient}>
        <Layout />
      </QueryClientProvider>
    </rspc.Provider>
  </React.StrictMode>
);
