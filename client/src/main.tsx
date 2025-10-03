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
  redirect_uri: `${window.location.origin}/callback`,
  scope: 'openid profile email',
  response_type: 'code',
  automaticSilentRenew: true,
};

function onSigninCallback() {
  window.history.replaceState({}, document.title, window.location.pathname);
}

// Wrapper component that initializes SpacetimeDB after auth
function AppWithSpacetime() {
  const auth = useAuth();

  // Wait for authentication
  if (auth.isLoading) {
    return <div>Loading authentication...</div>;
  }

  // If not authenticated, show App (which will show login button)
  if (!auth.isAuthenticated || !auth.user?.access_token) {
    return <App />;
  }

  // Initialize SpacetimeDB connection with auth token
  const spacetimeConfig = getSpacetimeConfig();
  const connection = DbConnection.builder()
    .withUri(spacetimeConfig.uri)
    .withModuleName(spacetimeConfig.moduleName)
    .withToken(auth.user.access_token)
    .onConnect((conn, identity, token) => {
      console.log('Connected to SpacetimeDB', { identity: identity.toHexString() });
      localStorage.setItem(spacetimeConfig.tokenKey, token);
    })
    .onConnectError((error) => {
      console.error('Failed to connect to SpacetimeDB:', error);
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