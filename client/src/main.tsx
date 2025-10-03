import { useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import { AuthProvider, useAuth } from 'react-oidc-context';
import { SpacetimeDBProvider } from './components/SpacetimeDBProvider';
import { DbConnection } from './autobindings';
import { getSpacetimeConfig } from './config/spacetime';
import App from './App';

const oidcConfig = {
  authority: 'https://auth.spacetimedb.com/oidc',
  client_id: 'client_031CSnBZhPFgz5oj5Alo0a',
  redirect_uri: `${window.location.origin}${import.meta.env.BASE_URL}`,
  scope: 'openid profile email',
  response_type: 'code',
  automaticSilentRenew: true,
};

function onSigninCallback() {
  // Clean up URL after OAuth callback
  window.history.replaceState(
    {},
    document.title,
    window.location.pathname
  );
}

// Wrapper component that initializes SpacetimeDB after auth
function AppWithSpacetime() {
  const auth = useAuth();

  useEffect(() => {
    console.log('🔐 Auth state:', {
      isLoading: auth.isLoading,
      isAuthenticated: auth.isAuthenticated,
      hasUser: !!auth.user,
      hasToken: !!auth.user?.access_token,
      error: auth.error?.message,
    });
  }, [auth.isLoading, auth.isAuthenticated, auth.user, auth.error]);

  // Wait for authentication
  if (auth.isLoading) {
    return <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100vh', color: 'white' }}>Loading authentication...</div>;
  }

  // If not authenticated, show App (which will show login button)
  if (!auth.isAuthenticated || !auth.user?.access_token) {
    return <App />;
  }

  // Extract username from JWT token
  const getUsername = () => {
    try {
      const decoded: any = auth.user?.profile || {};
      return decoded.preferred_username || decoded.name || decoded.email || decoded.sub || 'Player';
    } catch (e) {
      console.error('Failed to get username:', e);
      return 'Player';
    }
  };

  const username = getUsername();

  // Initialize SpacetimeDB connection with auth token
  const spacetimeConfig = getSpacetimeConfig();
  const connection = DbConnection.builder()
    .withUri(spacetimeConfig.uri)
    .withModuleName(spacetimeConfig.moduleName)
    .withToken(auth.user.access_token)
    .onConnect((conn, identity, token) => {
      console.log('✅ Connected to SpacetimeDB', {
        identity: identity.toHexString(),
        username
      });
      localStorage.setItem(spacetimeConfig.tokenKey, token);

      // Subscribe to all tables
      console.log('📡 Subscribing to SpacetimeDB tables...');
      conn.subscriptionBuilder()
          .subscribeToAllTables();
      // Register player immediately after connection is established
      console.log('Registering player with username:', username);
      conn.reducers.registerPlayer(username);
    })
    .onConnectError((error) => {
      console.error('❌ Failed to connect to SpacetimeDB:', error);
    })
    .build();

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

    const onUserLoaded = (u: any) => console.log("[OIDC] userLoaded", u?.profile?.sub, u);
    const onUserUnloaded = () => console.log("[OIDC] userUnloaded");
    const onAccessTokenExpiring = () => console.log("[OIDC] accessTokenExpiring");
    const onAccessTokenExpired = () => console.log("[OIDC] accessTokenExpired");
    const onSilentRenewError = (e: any) => console.warn("[OIDC] silentRenewError", e);
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
  }, [auth.isLoading, auth.isAuthenticated, auth.error, auth.activeNavigator, auth.user]);

  return null;
}

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

root.render(
    <AuthProvider {...oidcConfig} onSigninCallback={onSigninCallback}>
    <OidcDebug />
    <AppWithSpacetime />
  </AuthProvider>
);