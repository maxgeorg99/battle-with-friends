import React from "react";
import { useAuth } from "react-oidc-context";
import GameWrapper from "./components/GameWrapper";
import { getSpacetimeConfig } from "./config/spacetime";

const App: React.FC = () => {
  const spacetimeConfig = getSpacetimeConfig();

  // Try to get auth if it's available (only in production/staging)
  let isAuthenticated = true;
  let signinRedirect: (() => void) | undefined;

  try {
    const auth = useAuth();
    isAuthenticated = auth.isAuthenticated;
    signinRedirect = auth.signinRedirect;
  } catch (e) {
    // useAuth will throw if not wrapped in AuthProvider (local mode)
    // In local mode, we're always "authenticated"
    isAuthenticated = true;
  }

  // Show login button only if auth is required and user is not authenticated
  if (spacetimeConfig.requireAuth && !isAuthenticated) {
    return (
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          height: "100vh",
        }}
      >
        <h1>Battle with Friends</h1>
        <button
          onClick={() => signinRedirect?.()}
          style={{
            padding: "12px 24px",
            fontSize: "16px",
            cursor: "pointer",
            background: "#4CAF50",
            color: "white",
            border: "none",
            borderRadius: "4px",
          }}
        >
          Login to Play
        </button>
      </div>
    );
  }

  return <GameWrapper />;
};

export default App;
