import Phaser from 'phaser';
import { GameBridge } from '../GameBridge';
import { Player } from '../../autobindings';

export default class MainScene extends Phaser.Scene {
  private gameBridge!: GameBridge;
  private playerIdentity!: string;
  private localPlayer!: Phaser.GameObjects.Rectangle;
  private localPlayerText!: Phaser.GameObjects.Text;
  private otherPlayers: Mapy = new Map();
  private cursors!: Phaser.Types.Input.Keyboard.CursorKeys;
  private lastUpdateTime: number = 0;
  private updateThrottle: number = 50; // ms between updates

  constructor() {
    super('MainScene');
  }

  init() {
    this.gameBridge = this.registry.get('gameBridge');
    this.playerIdentity = this.registry.get('playerIdentity');
  }

  create() {
    // Background
    this.add.rectangle(400, 300, 800, 600, 0x1a1a1a);

    // Create local player
    this.localPlayer = this.add.rectangle(400, 300, 32, 32, 0x00ff00);
    this.localPlayerText = this.add.text(400, 280, 'You', {
      fontSize: '12px',
      color: '#ffffff'
    }).setOrigin(0.5);

    // Setup controls
    this.cursors = this.input.keyboard!.createCursorKeys();

    // Subscribe to player updates from React
    this.gameBridge.subscribe(() => {
      this.syncPlayers();
    });

    // Initial sync
    this.syncPlayers();
  }

  syncPlayers() {
    const players = this.gameBridge.getPlayers();
    const currentIdentities = new Set();

    players.forEach(player => {
      const identity = player.identity.toHexString();
      currentIdentities.add(identity);

      if (identity === this.playerIdentity) {
        // Skip local player - we control it directly
        return;
      }

      if (!this.otherPlayers.has(identity)) {
        // Create new player sprite
        const sprite = this.add.rectangle(player.x, player.y, 32, 32, 0xff0000);
        const text = this.add.text(player.x, player.y - 20, player.name, {
          fontSize: '12px',
          color: '#ffffff'
        }).setOrigin(0.5);

        this.otherPlayers.set(identity, { sprite, text });
      } else {
        // Update existing player
        const playerObj = this.otherPlayers.get(identity)!;
        playerObj.sprite.setPosition(player.x, player.y);
        playerObj.text.setPosition(player.x, player.y - 20);
        
        // Update opacity based on online status
        const alpha = player.online ? 1 : 0.3;
        playerObj.sprite.setAlpha(alpha);
        playerObj.text.setAlpha(alpha);
      }
    });

    // Remove players that left
    this.otherPlayers.forEach((playerObj, identity) => {
      if (!currentIdentities.has(identity)) {
        playerObj.sprite.destroy();
        playerObj.text.destroy();
        this.otherPlayers.delete(identity);
      }
    });
  }

  update(time: number) {
    const speed = 200;
    let moved = false;

    if (this.cursors.left.isDown) {
      this.localPlayer.x -= speed * this.game.loop.delta / 1000;
      moved = true;
    } else if (this.cursors.right.isDown) {
      this.localPlayer.x += speed * this.game.loop.delta / 1000;
      moved = true;
    }

    if (this.cursors.up.isDown) {
      this.localPlayer.y -= speed * this.game.loop.delta / 1000;
      moved = true;
    } else if (this.cursors.down.isDown) {
      this.localPlayer.y += speed * this.game.loop.delta / 1000;
      moved = true;
    }

    // Update text position
    this.localPlayerText.setPosition(this.localPlayer.x, this.localPlayer.y - 20);

    // Throttled position updates to server
    if (moved && time - this.lastUpdateTime > this.updateThrottle) {
      this.gameBridge.updatePosition(this.localPlayer.x, this.localPlayer.y);
      this.lastUpdateTime = time;
    }
  }
}