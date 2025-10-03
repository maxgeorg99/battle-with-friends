import { UserManager, User } from 'oidc-client-ts';
import { jwtDecode } from 'jwt-decode';
import Phaser from 'phaser';
import { DbConnection } from './autobindings';
import { getSpacetimeConfig } from './config/spacetime';
import MainScene from './game/scenes/MainScene';

// OIDC Configuration
const oidcConfig = {
  authority: 'https://auth.spacetimedb.com/oidc',
  client_id: 'client_031CSnBZhPFgz5oj5Alo0a',
  redirect_uri: `${window.location.origin}${import.meta.env.BASE_URL}`,
  scope: 'openid profile email',
  response_type: 'code',
  automaticSilentRenew: true,
};

const userManager = new UserManager(oidcConfig);

// Helper to extract username from JWT token
function getUsernameFromToken(user: User): string {
  try {
    const decoded: any = jwtDecode(user.access_token);
    const profile: any = user.profile || {};
    // Try common JWT claims for username
    return decoded.preferred_username || profile.preferred_username ||
           decoded.name || profile.name ||
           decoded.email || profile.email ||
           decoded.sub || profile.sub || 'Player';
  } catch (e) {
    console.error('Failed to decode JWT:', e);
    return 'Player';
  }
}

// Show login screen
function showLogin() {
  const app = document.getElementById('app')!;
  app.innerHTML = `
    <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background: #1a1a1a; color: white; font-family: system-ui;">
      <h1 style="margin-bottom: 2rem;">Battle with Friends</h1>
      <button id="login-btn" style="padding: 12px 24px; font-size: 16px; cursor: pointer; background: #4CAF50; color: white; border: none; border-radius: 4px;">
        Login to Play
      </button>
    </div>
  `;

  document.getElementById('login-btn')!.addEventListener('click', () => {
    userManager.signinRedirect();
  });
}

// Show loading screen
function showLoading(message: string = 'Loading...') {
  const app = document.getElementById('app')!;
  app.innerHTML = `
    <div style="display: flex; align-items: center; justify-content: center; height: 100vh; background: #1a1a1a; color: white; font-family: system-ui; font-size: 18px;">
      ${message}
    </div>
  `;
}

// Initialize game with authenticated user
async function initGame(user: User) {
  const username = getUsernameFromToken(user);
  console.log('🎮 Initializing game for user:', username);

  showLoading('Connecting to game server...');

  // Setup SpacetimeDB connection
  const spacetimeConfig = getSpacetimeConfig();

  return new Promise<DbConnection>((resolve, reject) => {
    const connection = DbConnection.builder()
      .withUri(spacetimeConfig.uri)
      .withModuleName(spacetimeConfig.moduleName)
      .withToken(user.access_token)
      .onConnect((conn, identity, token) => {
        console.log('✅ Connected to SpacetimeDB', {
          identity: identity.toHexString(),
          username
        });
        localStorage.setItem(spacetimeConfig.tokenKey, token);

        // Register player immediately after connection is established
        console.log('Registering player with username:', username);
        conn.reducers.registerPlayer(username);

        // Wait a moment for registration to complete
        setTimeout(() => {
          // Clear loading screen
          document.getElementById('app')!.innerHTML = '';

          // Initialize Phaser game
          const config: Phaser.Types.Core.GameConfig = {
            type: Phaser.AUTO,
            width: 800,
            height: 600,
            parent: 'app',
            backgroundColor: '#1a1a1a',
            physics: {
              default: 'arcade',
              arcade: {
                gravity: { y: 0 },
                debug: false,
              },
            },
            scene: MainScene,
          };

          const game = new Phaser.Game(config);

          // Pass connection and username to Phaser
          game.registry.set('connection', conn);
          game.registry.set('username', username);
          game.registry.set('localIdentity', identity.toHexString());

          console.log('🎮 Game started!');
          resolve(conn);
        }, 500);
      })
      .onConnectError((error) => {
        console.error('❌ Failed to connect to SpacetimeDB:', error);
        showLoading('Failed to connect to game server. Please refresh.');
        reject(error);
      })
      .build();
  });
}

// Handle OAuth callback
async function handleCallback() {
  try {
    const user = await userManager.signinRedirectCallback();
    // Clean up URL
    window.history.replaceState({}, document.title, window.location.pathname);
    return user;
  } catch (e) {
    console.error('OAuth callback error:', e);
    return null;
  }
}

// Main app initialization
async function main() {
  console.log('🚀 Starting Battle with Friends...');

  // Check if we're handling OAuth callback
  if (window.location.search.includes('code=')) {
    showLoading('Completing authentication...');
    const user = await handleCallback();
    if (user) {
      await initGame(user);
    } else {
      showLogin();
    }
    return;
  }

  // Try to get existing user session
  showLoading('Checking authentication...');
  const user = await userManager.getUser();

  if (user && !user.expired) {
    console.log('✅ User already authenticated');
    await initGame(user);
  } else {
    console.log('❌ Not authenticated');
    showLogin();
  }
}

main();
