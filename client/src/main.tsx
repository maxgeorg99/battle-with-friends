import React, { useEffect } from "react";
import ReactDOM from "react-dom/client";
import { AuthProvider, useAuth } from "react-oidc-context";
import { SpacetimeDBProvider } from "./components/SpacetimeDBProvider";
import { DbConnection } from "./autobindings";
import { getSpacetimeConfig } from "./config/spacetime";
import App from "./App";

const oidcConfig = {
  authority: "https://auth.spacetimedb.com/oidc",
  client_id: "client_031CSnBZhPFgz5oj5Alo0a",
  redirect_uri: `${window.location.origin}${import.meta.env.BASE_URL}`,
  scope: "openid profile email",
  response_type: "code",
  automaticSilentRenew: true,
};

function onSigninCallback() {
  // Clean up URL after OAuth callback
  window.history.replaceState({}, document.title, window.location.pathname);
}

// Local development wrapper without auth
function LocalAppWithSpacetime() {
  const [connection, setConnection] = React.useState<DbConnection | null>(null);
  const [username] = React.useState<string>("LocalPlayer");

  useEffect(() => {
    console.log("🔌 Creating local SpacetimeDB connection (no auth)");

    const spacetimeConfig = getSpacetimeConfig();
    const newConnection = DbConnection.builder()
      .withUri(spacetimeConfig.uri)
      .withModuleName(spacetimeConfig.moduleName)
      .onConnect((conn, identity, token) => {
        console.log("✅ Connected to local SpacetimeDB", {
          identity: identity.toHexString(),
          username,
        });
        localStorage.setItem(spacetimeConfig.tokenKey, token);

        // Subscribe to all tables
        console.log("📡 Subscribing to SpacetimeDB tables...");
        conn.subscriptionBuilder().subscribeToAllTables();

        // Register player immediately after connection is established
        console.log("Registering player with username:", username);
        conn.reducers.registerPlayer(username);
      })
      .onConnectError((error) => {
        console.error("❌ Failed to connect to SpacetimeDB:", error);
      })
      .build();

    setConnection(newConnection);

    return () => {
      if (newConnection) {
        console.log("🔌 Disconnecting from SpacetimeDB");
        newConnection.disconnect();
      }
    };
  }, [username]);

  // Wait for connection to be established
  if (!connection) {
    return (
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          height: "100vh",
          color: "white",
        }}
      >
        Connecting to local game server...
      </div>
    );
  }

  return (
    <SpacetimeDBProvider connection={connection}>
      <App />
    </SpacetimeDBProvider>
  );
}

// Wrapper component that initializes SpacetimeDB after auth (for staging/production)
function AppWithSpacetime() {
  const auth = useAuth();
  const [connection, setConnection] = React.useState<DbConnection | null>(null);
  const [username, setUsername] = React.useState<string>("Player");

  useEffect(() => {
    console.log("🔐 Auth state:", {
      isLoading: auth.isLoading,
      isAuthenticated: auth.isAuthenticated,
      hasUser: !!auth.user,
      hasToken: !!auth.user?.access_token,
      error: auth.error?.message,
    });
  }, [auth.isLoading, auth.isAuthenticated, auth.user, auth.error]);

  // Listen for token renewal and reconnect
  useEffect(() => {
    const handleUserLoaded = (user: any) => {
      console.log("🔄 Token refreshed, reconnecting to SpacetimeDB...");
      if (connection) {
        connection.disconnect();
      }
      // Trigger reconnection by setting connection to null
      setConnection(null);
    };

    auth.events.addUserLoaded(handleUserLoaded);

    return () => {
      auth.events.removeUserLoaded(handleUserLoaded);
    };
  }, [auth.events, connection]);

  // Initialize/reinitialize SpacetimeDB connection when token changes
  useEffect(() => {
    if (!auth.isAuthenticated || !auth.user?.access_token) {
      return;
    }

    // Extract username from JWT token
    const getUsername = () => {
      try {
        const decoded: any = auth.user?.profile || {};
        return (
          decoded.preferred_username ||
          decoded.name ||
          decoded.email ||
          decoded.sub ||
          "Player"
        );
      } catch (e) {
        console.error("Failed to get username:", e);
        return "Player";
      }
    };

    const currentUsername = getUsername();
    setUsername(currentUsername);

    console.log("🔌 Creating SpacetimeDB connection with token");

    const spacetimeConfig = getSpacetimeConfig();
    const newConnection = DbConnection.builder()
      .withUri(spacetimeConfig.uri)
      .withModuleName(spacetimeConfig.moduleName)
      .withToken(auth.user.access_token)
      .onConnect((conn, identity, token) => {
        console.log("✅ Connected to SpacetimeDB", {
          identity: identity.toHexString(),
          username: currentUsername,
        });
        localStorage.setItem(spacetimeConfig.tokenKey, token);

        // Subscribe to all tables
        console.log("📡 Subscribing to SpacetimeDB tables...");
        conn.subscriptionBuilder().subscribeToAllTables();

        // Register player immediately after connection is established
        console.log("Registering player with username:", currentUsername);
        conn.reducers.registerPlayer(currentUsername);
      })
      .onConnectError((error) => {
        console.error("❌ Failed to connect to SpacetimeDB:", error);
      })
      .build();

    setConnection(newConnection);

    return () => {
      if (newConnection) {
        console.log("🔌 Disconnecting from SpacetimeDB");
        newConnection.disconnect();
      }
    };
  }, [auth.user?.access_token, auth.isAuthenticated]);

  // Wait for authentication
  if (auth.isLoading) {
    return (
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          height: "100vh",
          color: "white",
        }}
      >
        Loading authentication...
      </div>
    );
  }

  // If not authenticated, show App (which will show login button)
  if (!auth.isAuthenticated || !auth.user?.access_token) {
    return <App />;
  }

  // Wait for connection to be established
  if (!connection) {
    return (
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          height: "100vh",
          color: "white",
        }}
      >
        Connecting to game server...
      </div>
    );
  }

  return (
    <SpacetimeDBProvider connection={connection}>
      <App />
    </SpacetimeDBProvider>
  );
}

export function OidcDebug() {
  const auth = useAuth();

  useEffect(() => {
    const ev = auth.events;

    const onUserLoaded = (u: any) =>
      console.log("[OIDC] userLoaded", u?.profile?.sub, u);
    const onUserUnloaded = () => console.log("[OIDC] userUnloaded");
    const onAccessTokenExpiring = () =>
      console.log("[OIDC] accessTokenExpiring");
    const onAccessTokenExpired = () => console.log("[OIDC] accessTokenExpired");
    const onSilentRenewError = (e: any) =>
      console.warn("[OIDC] silentRenewError", e);
    const onUserSignedOut = () => console.log("[OIDC] userSignedOut");

    ev.addUserLoaded(onUserLoaded);
    ev.addUserUnloaded(onUserUnloaded);
    ev.addAccessTokenExpiring(onAccessTokenExpiring);
    ev.addAccessTokenExpired(onAccessTokenExpired);
    ev.addSilentRenewError(onSilentRenewError);
    ev.addUserSignedOut(onUserSignedOut);

    return () => {
      ev.removeUserLoaded(onUserLoaded);
      ev.removeUserUnloaded(onUserUnloaded);
      ev.removeAccessTokenExpiring(onAccessTokenExpiring);
      ev.removeAccessTokenExpired(onAccessTokenExpired);
      ev.removeSilentRenewError(onSilentRenewError);
      ev.removeUserSignedOut(onUserSignedOut);
    };
  }, [auth.events]);

  useEffect(() => {
    console.log("[OIDC] state", {
      isLoading: auth.isLoading,
      isAuthenticated: auth.isAuthenticated,
      error: auth.error?.message,
      activeNavigator: auth.activeNavigator,
      user: !!auth.user,
    });
  }, [
    auth.isLoading,
    auth.isAuthenticated,
    auth.error,
    auth.activeNavigator,
    auth.user,
  ]);

  return null;
}

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement,
);

// Check if auth is required based on environment
const spacetimeConfig = getSpacetimeConfig();

if (spacetimeConfig.requireAuth) {
  // Production/Staging: Use OIDC authentication
  console.log("🔐 Auth required for environment:", spacetimeConfig.environment);
  root.render(
    <AuthProvider {...oidcConfig} onSigninCallback={onSigninCallback}>
      <OidcDebug />
      <AppWithSpacetime />
    </AuthProvider>,
  );
} else {
  // Local: Skip authentication
  console.log("🏠 Local development mode - skipping auth");
  root.render(<LocalAppWithSpacetime />);
}
