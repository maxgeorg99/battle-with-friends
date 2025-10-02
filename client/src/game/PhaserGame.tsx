import React, { useEffect, useRef } from 'react';
import Phaser from 'phaser';
import MainScene from './scenes/MainScene';
import { GameBridge } from './GameBridge';

interface PhaserGameProps {
  gameBridge: GameBridge;
  playerIdentity: string;
}

const PhaserGame: React.FC = ({ gameBridge, playerIdentity }) => {
  const gameRef = useRef(null);

  useEffect(() => {
    if (!gameRef.current) {
      const config: Phaser.Types.Core.GameConfig = {
        type: Phaser.AUTO,
        width: 800,
        height: 600,
        parent: 'root',
        physics: {
          default: 'arcade',
          arcade: {
            gravity: { y: 0 },
            debug: false,
          },
        },
        scene: MainScene,
      };

      gameRef.current = new Phaser.Game(config);
      
      // Pass bridge to Phaser
      gameRef.current.registry.set('gameBridge', gameBridge);
      gameRef.current.registry.set('playerIdentity', playerIdentity);
    }

    return () => {
      if (gameRef.current) {
        gameRef.current.destroy(true);
        gameRef.current = null;
      }
    };
  }, [gameBridge, playerIdentity]);

  return null;
};

export default PhaserGame;