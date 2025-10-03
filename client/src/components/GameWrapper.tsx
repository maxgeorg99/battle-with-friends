import React, { useEffect, useState, useMemo } from 'react';
import { useAuth } from 'react-oidc-context';
import { jwtDecode } from 'jwt-decode';
import { useSpacetimeDB } from './SpacetimeDBProvider';
import { Player } from '../autobindings';
import PhaserGame from '../game/PhaserGame';
import { GameBridge } from '../game/GameBridge';

const GameWrapper: React.FC = () => {
  const auth = useAuth();
  const conn = useSpacetimeDB();
  const [players, setPlayers] = useState<Player[]>([]);
  const [gameBridge] = useState(() => new GameBridge());
  const [registered, setRegistered] = useState(false);

  // Extract username from JWT token
  const username = useMemo(() => {
    if (!auth.user?.access_token) return 'Player';

    try {
      const decoded: any = jwtDecode(auth.user.access_token);
      // Try common JWT claims for username
      return decoded.preferred_username || decoded.name || decoded.email || decoded.sub || 'Player';
    } catch (e) {
      console.error('Failed to decode JWT:', e);
      return 'Player';
    }
  }, [auth.user?.access_token]);

  // Register player once connected
  useEffect(() => {
    if (conn && !registered && username) {
      console.log('Registering player with username:', username);
      conn.reducers.registerPlayer(username);
      setRegistered(true);
    }
  }, [conn, registered, username]);

  // Subscribe to player table updates
  useEffect(() => {
    if (conn) {
      const playerTable = conn.db.player;

      const updatePlayers = () => {
        const allPlayers = Array.from(playerTable.iter());
        setPlayers(allPlayers);
      };

      // Initial load
      updatePlayers();

      // Listen for updates
      playerTable.onInsert(updatePlayers);
      playerTable.onUpdate(updatePlayers);
      playerTable.onDelete(updatePlayers);
    }
  }, [conn]);

  // Sync players to game via bridge
  useEffect(() => {
    if (players.length > 0 && conn?.identity) {
      gameBridge.updatePlayers(players, conn.identity.toHexString());
    }
  }, [players, conn?.identity, gameBridge]);

  // Setup position update callback from Phaser
  useEffect(() => {
    if (conn) {
      gameBridge.onPositionUpdate = (x: number, y: number) => {
        conn.reducers.updatePosition(x, y);
      };
    }
  }, [conn, gameBridge]);

  if (!conn?.identity) {
    return <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100vh', color: 'white' }}>Connecting to game server...</div>;
  }

  return <PhaserGame bridge={gameBridge} />;
};

export default GameWrapper;