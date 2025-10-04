import Phaser from 'phaser';
import { DbConnection } from '../../autobindings';
import { Identity } from 'spacetimedb';
import { World } from '../ecs/World';
import { DragDropSystem } from '../systems/DragDropSystem';
import { ComponentTypes, CrewData, ShopCrewData, GridPosition, Sprite, Draggable } from '../ecs/Components';

export default class ShipScene extends Phaser.Scene {
  private connection!: DbConnection;
  private localIdentity!: Identity;
  private username!: string;
  private world!: World;

  // UI elements
  private berriesText!: Phaser.GameObjects.Text;
  private bountyText!: Phaser.GameObjects.Text;
  private shipTypeText!: Phaser.GameObjects.Text;
  private shopContainer!: Phaser.GameObjects.Container;

  constructor() {
    super('ShipScene');
  }

  preload() {
    console.log('🔄 ShipScene preload started');

    const baseUrl = import.meta.env.BASE_URL || '/';

    // Load crew portraits for shop
    this.load.image('shop-luffy', `${baseUrl}assets/shop/icon/Luffy.JPG`);
    this.load.image('shop-lysop', `${baseUrl}assets/shop/icon/Lysop.JPG`);
    this.load.image('shop-nami', `${baseUrl}assets/shop/icon/Nami.JPG`);
    this.load.image('shop-zoro', `${baseUrl}assets/shop/icon/Zoro.JPG`);

    // Load unit sprites for field
    this.load.image('unit-luffy', `${baseUrl}assets/unit/Luffy.PNG`);
    this.load.image('unit-lysop', `${baseUrl}assets/unit/Lysop.PNG`);
    this.load.image('unit-nami', `${baseUrl}assets/unit/Nami.PNG`);
    this.load.image('unit-zoro', `${baseUrl}assets/unit/Zoro.PNG`);

    // Load trait icons
    this.load.image('trait-marines', `${baseUrl}assets/trait/Marines.png`);
    this.load.image('trait-logia', `${baseUrl}assets/trait/Logia.png`);
    this.load.image('trait-paramecia', `${baseUrl}assets/trait/Paramecia.png`);
    this.load.image('trait-zoan', `${baseUrl}assets/trait/Zoan.png`);

    // Add load event listeners
    this.load.on('complete', () => {
      console.log('✅ ShipScene: All assets loaded successfully');
    });

    this.load.on('loaderror', (fileObj: any) => {
      console.error('❌ ShipScene: Error loading asset:', fileObj.key, fileObj.url);
    });
  }

  init() {
    this.connection = this.registry.get('connection');
    this.username = this.registry.get('username');
    this.localIdentity = this.registry.get('localIdentity');

    console.log('🚢 ShipScene initialized', {
      hasConnection: !!this.connection,
      hasIdentity: !!this.localIdentity,
      identityValue: this.localIdentity,
      username: this.username
    });

    if (!this.connection || !this.localIdentity) {
      console.error('❌ Missing connection or identity!', {
        connection: this.connection,
        localIdentity: this.localIdentity
      });
      return;
    }

    // Initialize ECS
    this.world = new World();
    this.world.addSystem(new DragDropSystem(this));
  }

  create() {
    console.log('🎨 ShipScene create() started');

    // Guard against missing connection/identity
    if (!this.connection || !this.localIdentity) {
      console.error('❌ Cannot create scene without connection/identity');
      return;
    }

    // Background
    this.add.rectangle(512, 384, 1024, 768, 0x4a90e2); // Ocean blue

    // Create UI
    this.createUI();

    // Create ship grid
    this.createShipGrid();

    // Subscribe to SpacetimeDB updates
    this.subscribeToDatabase();

    // Listen for crew movement events
    this.events.on('crew-moved', this.onCrewMoved, this);

    // Initial data load
    this.loadPlayerData();
    this.loadCrewData();
    this.loadShopData();

    console.log('✅ ShipScene create() completed');
  }

  update(time: number, delta: number) {
    this.world.update(delta);
  }

  private createUI() {
    // Ship grid label (RAFT) at top
    const raftLabel = this.add.rectangle(512, 80, 200, 60, 0xd4b896)
      .setStrokeStyle(3, 0x3d2817);

    this.add.text(512, 80, 'RAFT', {
      fontSize: '32px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // RECRUIT banner below ship grid
    const recruitBanner = this.add.rectangle(512, 450, 450, 65, 0xd4b896)
      .setStrokeStyle(4, 0x3d2817);

    this.add.text(512, 450, 'RECRUIT', {
      fontSize: '40px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    this.shopContainer = this.add.container(0, 0);

    // Bottom left - Level/Round indicator
    const levelBg = this.add.circle(105, 680, 50, 0xd4b896)
      .setStrokeStyle(4, 0x3d2817);

    this.add.text(105, 655, '8', {
      fontSize: '48px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    const berriesIcon = this.add.circle(105, 705, 15, 0xffd700)
      .setStrokeStyle(3, 0x3d2817);

    // Bottom right - Refresh button
    const refreshBg = this.add.rectangle(895, 680, 200, 60, 0xd4b896)
      .setStrokeStyle(4, 0x3d2817);
    refreshBg.setInteractive(new Phaser.Geom.Rectangle(-100, -30, 200, 60), Phaser.Geom.Rectangle.Contains);
    refreshBg.input!.cursor = 'pointer';
    refreshBg.on('pointerdown', () => this.refreshShop());

    this.add.text(895, 680, 'REFRESH', {
      fontSize: '24px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // Store text references (not displayed in this minimal UI like screenshot)
    this.berriesText = this.add.text(0, 0, '', { fontSize: '1px' }).setVisible(false);
    this.bountyText = this.add.text(0, 0, '', { fontSize: '1px' }).setVisible(false);
    this.shipTypeText = this.add.text(0, 0, '', { fontSize: '1px' }).setVisible(false);
  }

  private createShipGrid() {
    const gridStartX = 332;  // Centered for 5 columns
    const gridStartY = 180;  // Below RAFT label
    const cellSize = 64;
    const cols = 5;
    const rows = 3;

    // Draw ship grid (3x5 grid) - wooden deck style
    for (let row = 0; row < rows; row++) {
      for (let col = 0; col < cols; col++) {
        const x = gridStartX + col * cellSize;
        const y = gridStartY + row * cellSize;

        // Grid cell background - wooden planks
        this.add.rectangle(x, y, cellSize - 2, cellSize - 2, 0xa0826d)
          .setStrokeStyle(3, 0x654321);
      }
    }
  }

  private subscribeToDatabase() {
    if (!this.connection || !this.localIdentity) return;

    // Subscribe to player updates
    this.connection.db.player.onUpdate((oldPlayer, newPlayer) => {
      if (this.localIdentity && newPlayer.identity.isEqual(this.localIdentity)) {
        this.updatePlayerUI(newPlayer);
      }
    });

    // Subscribe to crew updates
    this.connection.db.crew.onInsert((crew) => {
      if (this.localIdentity && crew.owner.isEqual(this.localIdentity)) {
        this.addCrewEntity(crew);
      }
    });

    this.connection.db.crew.onUpdate((oldCrew, newCrew) => {
      if (this.localIdentity && newCrew.owner.isEqual(this.localIdentity)) {
        this.updateCrewEntity(newCrew);
      }
    });

    this.connection.db.crew.onDelete((crew) => {
      if (this.localIdentity && crew.owner.isEqual(this.localIdentity)) {
        this.removeCrewEntity(crew.id);
      }
    });

    // Subscribe to shop updates
    this.connection.db.shopCrew.onInsert((shopCrew) => {
      if (this.localIdentity && shopCrew.player && shopCrew.player.isEqual(this.localIdentity)) {
        this.addShopCrewCard(shopCrew);
      }
    });

    this.connection.db.shopCrew.onDelete((shopCrew) => {
      if (this.localIdentity && shopCrew.player && shopCrew.player.isEqual(this.localIdentity)) {
        this.removeShopCrewCard(shopCrew.id);
      }
    });
  }

  private loadPlayerData() {
    if (!this.connection || !this.localIdentity) return;

    const player = Array.from(this.connection.db.player.iter())
      .find(p => p.identity.isEqual(this.localIdentity));

    if (player) {
      this.updatePlayerUI(player);
    }
  }

  private loadCrewData() {
    if (!this.connection || !this.localIdentity) return;

    const crewList = Array.from(this.connection.db.crew.iter())
      .filter(c => c.owner.isEqual(this.localIdentity));

    for (const crew of crewList) {
      this.addCrewEntity(crew);
    }
  }

  private loadShopData() {
    if (!this.connection || !this.localIdentity) return;

    const shopList = Array.from(this.connection.db.shopCrew.iter())
      .filter(s => s.player && s.player.isEqual(this.localIdentity));

    for (const shopCrew of shopList) {
      this.addShopCrewCard(shopCrew);
    }
  }

  private updatePlayerUI(player: any) {
    this.berriesText.setText(`Berries: ${player.berries}`);
    this.bountyText.setText(`Bounty: ${player.bounty}`);
    this.shipTypeText.setText(`Ship: ${player.shipType}`);
  }

  private addCrewEntity(crew: any) {
    const entity = this.world.createEntity();

    // Add crew data component
    const crewData: CrewData = {
      id: Number(crew.id),
      name: crew.name,
      rarity: crew.rarity,
      trait1: crew.trait1,
      trait2: crew.trait2,
      maxHp: crew.maxHp,
      currentHp: crew.currentHp,
      attack: crew.attack,
      defense: crew.defense,
      level: crew.level,
    };
    entity.addComponent(ComponentTypes.CREW_DATA, crewData);

    // Create sprite
    const container = this.createCrewCard(crew, false);
    entity.addComponent(ComponentTypes.SPRITE, { gameObject: container });

    // Add grid position
    if (crew.slotIndex !== null && crew.slotIndex !== undefined) {
      const gridPos: GridPosition = {
        x: crew.slotIndex,
        y: 0,
        width: 1,
        height: 1,
      };
      entity.addComponent(ComponentTypes.GRID_POSITION, gridPos);

      // Position on grid
      const screenPos = this.gridToScreen(gridPos.x, gridPos.y);
      container.setPosition(screenPos.x, screenPos.y);
    }

    // Make draggable
    const draggable: Draggable = {
      isDragging: false,
      originalPosition: { x: container.x, y: container.y },
    };
    entity.addComponent(ComponentTypes.DRAGGABLE, draggable);
  }

  private createCrewCard(crew: any, isShopCard: boolean): Phaser.GameObjects.Container {
    const container = this.add.container(0, 0);

    // Map crew name to sprite key
    const getCrewSpriteKey = (name: string): string => {
      const nameLower = name.toLowerCase().replace(/\s+/g, '');
      if (nameLower.includes('luffy')) return isShopCard ? 'shop-luffy' : 'unit-luffy';
      if (nameLower.includes('zoro')) return isShopCard ? 'shop-zoro' : 'unit-zoro';
      if (nameLower.includes('nami')) return isShopCard ? 'shop-nami' : 'unit-nami';
      if (nameLower.includes('usopp') || nameLower.includes('lysop')) return isShopCard ? 'shop-lysop' : 'unit-lysop';
      return ''; // Fallback - will show colored rectangle
    };

    const spriteKey = getCrewSpriteKey(crew.name);

    // Card background based on rarity
    const rarityColors: Record<string, number> = {
      Common: 0x87ceeb,
      Rare: 0x4169e1,
      Epic: 0x9370db,
      Legendary: 0xffd700,
    };

    if (isShopCard) {
      // Shop card style - like screenshot
      const cardWidth = 110;
      const cardHeight = 140;

      // Parchment background
      const cardBg = this.add.rectangle(0, 0, cardWidth, cardHeight, 0xd4b896)
        .setStrokeStyle(4, rarityColors[crew.rarity] || 0x87ceeb);
      container.add(cardBg);

      if (spriteKey && this.textures.exists(spriteKey)) {
        // Portrait in upper portion
        const portrait = this.add.image(0, -20, spriteKey);
        portrait.setDisplaySize(90, 75);
        container.add(portrait);
      } else {
        // Fallback colored rectangle
        const portrait = this.add.rectangle(0, -20, 90, 75, rarityColors[crew.rarity] || 0x87ceeb);
        container.add(portrait);
      }

      // Crew name at bottom
      const nameText = this.add.text(0, 45, crew.name.toUpperCase(), {
        fontSize: '14px',
        color: '#3d2817',
        fontFamily: 'Georgia, serif',
        fontStyle: 'bold',
        align: 'center',
      }).setOrigin(0.5);
      container.add(nameText);

      // Cost at bottom with coin icon
      const costBg = this.add.circle(0, 65, 12, 0xffd700)
        .setStrokeStyle(2, 0x3d2817);
      container.add(costBg);

      const costText = this.add.text(0, 65, `${crew.cost || 1}`, {
        fontSize: '14px',
        color: '#3d2817',
        fontFamily: 'Georgia, serif',
        fontStyle: 'bold',
      }).setOrigin(0.5);
      container.add(costText);
    } else {
      // Field unit card - simpler
      if (spriteKey && this.textures.exists(spriteKey)) {
        const sprite = this.add.image(0, 0, spriteKey);
        sprite.setDisplaySize(50, 50);
        container.add(sprite);

        // Border
        const border = this.add.rectangle(0, 0, 50, 50)
          .setStrokeStyle(3, rarityColors[crew.rarity] || 0x87ceeb)
          .setFillStyle(0x000000, 0);
        container.add(border);
      } else {
        const bg = this.add.rectangle(0, 0, 50, 50, rarityColors[crew.rarity] || 0x87ceeb)
          .setStrokeStyle(2, 0x3d2817);
        container.add(bg);
      }
    }

    return container;
  }

  private addShopCrewCard(shopCrew: any) {
    // Create shop card
    const card = this.createCrewCard(shopCrew, true);

    // Position in shop - below grid, centered horizontally with spacing
    const shopIndex = this.shopContainer.list.length;
    const cardSpacing = 125;
    const startX = 290; // Start position for first card
    const x = startX + shopIndex * cardSpacing;
    card.setPosition(x, 550); // Below the RECRUIT banner

    // Make clickable to buy
    card.setSize(110, 140);
    card.setInteractive(new Phaser.Geom.Rectangle(-55, -70, 110, 140), Phaser.Geom.Rectangle.Contains);
    card.input!.cursor = 'pointer';
    card.on('pointerdown', () => this.buyCrewFromShop(shopCrew.id));

    this.shopContainer.add(card);
  }

  private removeShopCrewCard(shopCrewId: bigint) {
    // Remove from shop container
    this.shopContainer.removeAll(true);
  }

  private updateCrewEntity(crew: any) {
    // Find entity by crew ID and update it
    // For now, just recreate
    this.removeCrewEntity(crew.id);
    this.addCrewEntity(crew);
  }

  private removeCrewEntity(crewId: bigint) {
    const entities = this.world.queryEntities([ComponentTypes.CREW_DATA]);
    for (const entity of entities) {
      const crewData = entity.getComponent<CrewData>(ComponentTypes.CREW_DATA);
      if (crewData && crewData.id === Number(crewId)) {
        const sprite = entity.getComponent<Sprite>(ComponentTypes.SPRITE);
        if (sprite) {
          sprite.gameObject.destroy();
        }
        this.world.destroyEntity(entity.id);
        break;
      }
    }
  }

  private gridToScreen(gridX: number, gridY: number): { x: number; y: number } {
    const gridStartX = 332;
    const gridStartY = 180;
    const cellSize = 64;
    return {
      x: gridStartX + gridX * cellSize,
      y: gridStartY + gridY * cellSize,
    };
  }

  private onCrewMoved(entityId: number, gridPos: GridPosition) {
    console.log('Crew moved to grid position:', gridPos);
    // TODO: Call connection.reducers.moveCrew(crewId, gridPos.x)
  }

  private buyCrewFromShop(shopCrewId: bigint) {
    if (!this.connection || !this.localIdentity) return;

    console.log('Buying crew from shop:', shopCrewId);
    // Find empty slot
    const occupiedSlots = Array.from(this.connection.db.crew.iter())
      .filter(c => c.owner.isEqual(this.localIdentity) && c.slotIndex !== null)
      .map(c => c.slotIndex);

    for (let i = 0; i < 10; i++) {
      if (!occupiedSlots.includes(i)) {
        this.connection.reducers.buyCrew(shopCrewId, i);
        return;
      }
    }

    console.warn('No empty slots!');
  }

  private refreshShop() {
    this.connection.reducers.refreshShop();
  }

  private startBattle() {
    this.connection.reducers.startBattle();
    // TODO: Switch to battle scene
  }
}
