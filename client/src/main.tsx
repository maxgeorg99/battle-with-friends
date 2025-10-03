import { useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import { AuthProvider, useAuth } from 'react-oidc-context';
import { DbConnection } from './autobindings';
import { getSpacetimeConfig, getStoredToken } from './config/spacetime';
import PhaserGame from './game/PhaserGame';

// Get environment-specific SpacetimeDB configuration
const spacetimeConfig = getSpacetimeConfig();

const oidcConfig = {
  authority: 'https://spacetimeauth.staging.spacetimedb.com/oidc',
  client_id: 'client_031CSnBZhPFgz5oj5Alo0a',
  redirect_uri: `${window.location.origin}/callback`,
  scope: 'openid profile email',
  response_type: 'code',
  automaticSilentRenew: true,
};

function onSigninCallback() {
  window.history.replaceState({}, document.title, window.location.pathname);
}

// SpacetimeDB connection builder with environment-specific config
const connectionBuilder = DbConnection.builder()
  .withUri(spacetimeConfig.uri)
  .withModuleName(spacetimeConfig.moduleName)
  .withToken(getStoredToken() || '');

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
  <AuthProvider
    configuration={oidcConfig}
    onEvent={console.log}
    onSigninCallback={onSigninCallback}
  >
    <OidcDebug />
    <SpacetimeDBProvider connection={connectionBuilder.build()}>
      <PhaserGame />
    </SpacetimeDBProvider>
  </AuthProvider>
);