import React, { useEffect, useState } from 'react';
import { useSpacetimeDBConnection } from './SpacetimeDBProvider';
import { Player } from '../autobindings';
import PhaserGame from '../game/PhaserGame';
import { GameBridge } from '../game/GameBridge';

const GameWrapper: React.FC = () => {
  const { connection: conn, isConnected } = useSpacetimeDBConnection();
  const [players, setPlayers] = useState<Player[]>([]);
  const [gameBridge] = useState(() => new GameBridge());

  // Subscribe to player table updates
  useEffect(() => {
    if (conn && isConnected) {
      const playerTable = conn.db.player;

      const updatePlayers = () => {
        const allPlayers = Array.from(playerTable.iter());
        console.log('👥 Players updated:', allPlayers.length);
        setPlayers(allPlayers);
      };

      // Initial load
      updatePlayers();

      // Listen for updates
      playerTable.onInsert(updatePlayers);
      playerTable.onUpdate(updatePlayers);
      playerTable.onDelete(updatePlayers);
    }
  }, [conn, isConnected]);

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

  if (!isConnected) {
    return <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100vh', color: 'white' }}>Connecting to game server...</div>;
  }

  return <PhaserGame bridge={gameBridge} />;
};

export default GameWrapper;