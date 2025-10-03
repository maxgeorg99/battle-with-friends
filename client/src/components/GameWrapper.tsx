import React, { useEffect, useState } from 'react';
import { useAuth } from 'react-oidc-context';
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

  // Register player once connected
  useEffect(() => {
    if (conn && !registered && auth.user?.profile?.name) {
      conn.reducers.registerPlayer(auth.user.profile.name as string);
      setRegistered(true);
    }
  }, [conn, registered, auth.user?.profile?.name]);

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