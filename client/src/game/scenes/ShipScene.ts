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
  private berriesDisplayText!: Phaser.GameObjects.Text;
  private bountyText!: Phaser.GameObjects.Text;
  private shipTypeText!: Phaser.GameObjects.Text;
  private shopContainer!: Phaser.GameObjects.Container;
  private statsTooltip!: Phaser.GameObjects.Container;
  private chestOverlay!: Phaser.GameObjects.Container;
  private isChestOpen: boolean = false;

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

    // Create stats tooltip (initially hidden)
    this.createStatsTooltip();

    // Create chest overlay (initially hidden)
    this.createChestOverlay();

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

    // Bottom left - Berries display
    const berriesBg = this.add.circle(105, 680, 50, 0xd4b896)
      .setStrokeStyle(4, 0x3d2817);

    // Berry amount (will be updated)
    this.berriesDisplayText = this.add.text(105, 670, '1.0M', {
      fontSize: '20px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // Berry icon (₿ symbol)
    this.add.text(105, 695, '₿', {
      fontSize: '24px',
      color: '#ffd700',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // Top right - Treasure Chest button
    const chestBg = this.add.rectangle(920, 80, 100, 100, 0x8b6f47)
      .setStrokeStyle(4, 0x3d2817);
    chestBg.setInteractive(new Phaser.Geom.Rectangle(-50, -50, 100, 100), Phaser.Geom.Rectangle.Contains);
    chestBg.input!.cursor = 'pointer';
    chestBg.on('pointerdown', () => this.openChest());

    // Chest icon (simple treasure chest representation)
    this.add.rectangle(920, 80, 70, 50, 0xa0826d)
      .setStrokeStyle(3, 0x3d2817);
    this.add.rectangle(920, 65, 70, 10, 0xffd700)
      .setStrokeStyle(2, 0x3d2817);
    this.add.circle(920, 80, 8, 0xffd700)
      .setStrokeStyle(2, 0x3d2817);

    this.add.text(920, 110, 'CHEST', {
      fontSize: '12px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);

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
      if (this.localIdentity && crew.owner && crew.owner.isEqual(this.localIdentity)) {
        this.addCrewEntity(crew);
      }
    });

    this.connection.db.crew.onUpdate((oldCrew, newCrew) => {
      if (this.localIdentity && newCrew.owner && newCrew.owner.isEqual(this.localIdentity)) {
        this.updateCrewEntity(newCrew);
      }
    });

    this.connection.db.crew.onDelete((crew) => {
      if (this.localIdentity && crew.owner && crew.owner.isEqual(this.localIdentity)) {
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
      .filter(c => c.owner && c.owner.isEqual(this.localIdentity));

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

    // Update the visible berries display in bottom left
    if (this.berriesDisplayText) {
      this.berriesDisplayText.setText(this.formatBerries(player.berries));
    }
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
      // Convert slot index (0-14) to grid coordinates (3x5 grid)
      const slotIndex = Number(crew.slotIndex);
      const gridPos: GridPosition = {
        x: slotIndex % 5,  // Column (0-4)
        y: Math.floor(slotIndex / 5),  // Row (0-2)
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

    // Add hover events to show stats tooltip
    container.on('pointerover', () => {
      this.showStatsTooltip(crew, container.x, container.y);
    });

    container.on('pointerout', () => {
      this.hideStatsTooltip();
    });
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
      const nameText = this.add.text(0, 32, crew.name.toUpperCase(), {
        fontSize: '12px',
        color: '#3d2817',
        fontFamily: 'Georgia, serif',
        fontStyle: 'bold',
        align: 'center',
        wordWrap: { width: 100 },
      }).setOrigin(0.5);
      container.add(nameText);

      // Traits display
      const trait1Name = this.getTraitDisplayName(crew.trait1);
      const trait2Name = crew.trait2 ? this.getTraitDisplayName(crew.trait2) : null;

      const traitsText = trait2Name ? `${trait1Name} • ${trait2Name}` : trait1Name;
      const traits = this.add.text(0, 50, traitsText, {
        fontSize: '9px',
        color: '#654321',
        fontFamily: 'Georgia, serif',
        align: 'center',
        wordWrap: { width: 100 },
      }).setOrigin(0.5);
      container.add(traits);

      // Cost at bottom with formatted berries
      const formattedCost = this.formatBerries(crew.cost || 100000);
      const costText = this.add.text(0, 65, `₿${formattedCost}`, {
        fontSize: '12px',
        color: '#3d2817',
        fontFamily: 'Georgia, serif',
        fontStyle: 'bold',
        backgroundColor: '#ffd700',
        padding: { x: 6, y: 2 },
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

    console.log('💰 Buying crew from shop:', shopCrewId);
    // Find empty slot on 3x5 grid (15 total slots)
    const occupiedSlots = Array.from(this.connection.db.crew.iter())
      .filter(c => c.owner && c.owner.isEqual(this.localIdentity) && c.slotIndex !== null)
      .map(c => c.slotIndex);

    // Find first empty slot (0-14 for 3x5 grid)
    for (let slot = 0; slot < 15; slot++) {
      if (!occupiedSlots.includes(slot)) {
        console.log('🎯 Found empty slot:', slot);
        this.connection.reducers.buyCrew(shopCrewId, slot);
        return;
      }
    }

    console.warn('⚠️ No empty slots on the raft!');
  }

  private refreshShop() {
    if (!this.connection) {
      console.error('❌ No connection to refresh shop');
      return;
    }
    console.log('🔄 Refreshing shop...');
    this.connection.reducers.refreshShop();
  }

  private startBattle() {
    this.connection.reducers.startBattle();
    // TODO: Switch to battle scene
  }

  private getTraitDisplayName(trait: any): string {
    // Trait is an object with a 'tag' property
    const traitTag = trait?.tag || trait;

    const traitMap: Record<string, string> = {
      'StrawHat': 'Straw Hat',
      'Marine': 'Marine',
      'Revolutionary': 'Revolutionary',
      'Warlord': 'Warlord',
      'Emperor': 'Emperor',
      'Supernova': 'Supernova',
      'DFUser': 'DF User',
    };
    return traitMap[traitTag] || traitTag;
  }

  private formatBerries(amount: number): string {
    if (amount >= 1000000) {
      return `${(amount / 1000000).toFixed(1)}M`;
    } else if (amount >= 1000) {
      return `${(amount / 1000).toFixed(0)}K`;
    }
    return amount.toString();
  }

  private createStatsTooltip() {
    this.statsTooltip = this.add.container(0, 0);
    this.statsTooltip.setDepth(10000); // Always on top
    this.statsTooltip.setVisible(false);
  }

  private showStatsTooltip(crew: any, x: number, y: number) {
    // Clear previous tooltip content
    this.statsTooltip.removeAll(true);

    const tooltipWidth = 200;
    const tooltipHeight = 140;

    // Position tooltip to the right of the unit, or left if too close to edge
    const tooltipX = x + 80 > 1024 - tooltipWidth ? x - 80 : x + 80;
    const tooltipY = y;

    // Background
    const bg = this.add.rectangle(0, 0, tooltipWidth, tooltipHeight, 0x2c2416, 0.95)
      .setStrokeStyle(3, 0xd4b896);
    this.statsTooltip.add(bg);

    // Crew name
    const nameText = this.add.text(0, -55, crew.name, {
      fontSize: '14px',
      color: '#ffd700',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
      align: 'center',
      wordWrap: { width: tooltipWidth - 20 },
    }).setOrigin(0.5);
    this.statsTooltip.add(nameText);

    // Traits
    const trait1Name = this.getTraitDisplayName(crew.trait1);
    const trait2Name = crew.trait2 ? this.getTraitDisplayName(crew.trait2) : null;
    const traitsText = trait2Name ? `${trait1Name} • ${trait2Name}` : trait1Name;

    const traits = this.add.text(0, -25, traitsText, {
      fontSize: '10px',
      color: '#87ceeb',
      fontFamily: 'Georgia, serif',
      align: 'center',
      wordWrap: { width: tooltipWidth - 20 },
    }).setOrigin(0.5);
    this.statsTooltip.add(traits);

    // Stats
    const statsY = -5;
    const lineHeight = 16;

    const hpText = this.add.text(-80, statsY, `HP:`, {
      fontSize: '11px',
      color: '#d4b896',
      fontFamily: 'Georgia, serif',
    });
    this.statsTooltip.add(hpText);

    const hpValue = this.add.text(80, statsY, `${crew.currentHp || crew.maxHp}/${crew.maxHp}`, {
      fontSize: '11px',
      color: '#fff',
      fontFamily: 'Georgia, serif',
    }).setOrigin(1, 0);
    this.statsTooltip.add(hpValue);

    const atkText = this.add.text(-80, statsY + lineHeight, `Attack:`, {
      fontSize: '11px',
      color: '#d4b896',
      fontFamily: 'Georgia, serif',
    });
    this.statsTooltip.add(atkText);

    const atkValue = this.add.text(80, statsY + lineHeight, `${crew.attack}`, {
      fontSize: '11px',
      color: '#fff',
      fontFamily: 'Georgia, serif',
    }).setOrigin(1, 0);
    this.statsTooltip.add(atkValue);

    const defText = this.add.text(-80, statsY + lineHeight * 2, `Defense:`, {
      fontSize: '11px',
      color: '#d4b896',
      fontFamily: 'Georgia, serif',
    });
    this.statsTooltip.add(defText);

    const defValue = this.add.text(80, statsY + lineHeight * 2, `${crew.defense}`, {
      fontSize: '11px',
      color: '#fff',
      fontFamily: 'Georgia, serif',
    }).setOrigin(1, 0);
    this.statsTooltip.add(defValue);

    const levelText = this.add.text(-80, statsY + lineHeight * 3, `Level:`, {
      fontSize: '11px',
      color: '#d4b896',
      fontFamily: 'Georgia, serif',
    });
    this.statsTooltip.add(levelText);

    const levelValue = this.add.text(80, statsY + lineHeight * 3, `${crew.level || 1}`, {
      fontSize: '11px',
      color: '#fff',
      fontFamily: 'Georgia, serif',
    }).setOrigin(1, 0);
    this.statsTooltip.add(levelValue);

    // Items display
    if (crew.completedItem) {
      const itemName = this.getCompletedItemName(crew.completedItem);
      const itemText = this.add.text(0, statsY + lineHeight * 4 + 5, `⚔️ ${itemName}`, {
        fontSize: '11px',
        color: '#ffd700',
        fontFamily: 'Georgia, serif',
        fontStyle: 'bold',
        align: 'center',
      }).setOrigin(0.5, 0);
      this.statsTooltip.add(itemText);
    } else if (crew.item1 || crew.item2) {
      let itemsText = '🎒 ';
      if (crew.item1) {
        itemsText += this.getItemComponentName(crew.item1);
      }
      if (crew.item2) {
        itemsText += ` + ${this.getItemComponentName(crew.item2)}`;
      }
      const itemText = this.add.text(0, statsY + lineHeight * 4 + 5, itemsText, {
        fontSize: '10px',
        color: '#87ceeb',
        fontFamily: 'Georgia, serif',
        align: 'center',
      }).setOrigin(0.5, 0);
      this.statsTooltip.add(itemText);
    }

    // Position and show
    this.statsTooltip.setPosition(tooltipX, tooltipY);
    this.statsTooltip.setVisible(true);
  }

  private hideStatsTooltip() {
    this.statsTooltip.setVisible(false);
  }

  private createChestOverlay() {
    this.chestOverlay = this.add.container(0, 0);
    this.chestOverlay.setDepth(9999); // Below tooltip but above everything else
    this.chestOverlay.setVisible(false);
  }

  private openChest() {
    if (this.isChestOpen) {
      this.closeChest();
      return;
    }

    console.log('📦 Opening treasure chest...');
    this.isChestOpen = true;

    // Clear previous content
    this.chestOverlay.removeAll(true);

    // Semi-transparent dark background overlay
    const overlay = this.add.rectangle(512, 384, 1024, 768, 0x000000, 0.7);
    overlay.setInteractive();
    overlay.on('pointerdown', () => this.closeChest());
    this.chestOverlay.add(overlay);

    // Main chest panel
    const panelWidth = 800;
    const panelHeight = 600;
    const panelBg = this.add.rectangle(512, 384, panelWidth, panelHeight, 0xd4b896)
      .setStrokeStyle(6, 0x3d2817);
    this.chestOverlay.add(panelBg);

    // Title
    const titleBg = this.add.rectangle(512, 120, 400, 70, 0x8b6f47)
      .setStrokeStyle(4, 0x3d2817);
    this.chestOverlay.add(titleBg);

    const title = this.add.text(512, 120, 'TREASURE CHEST', {
      fontSize: '36px',
      color: '#ffd700',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);
    this.chestOverlay.add(title);

    // Close button
    const closeBtn = this.add.rectangle(850, 120, 60, 60, 0xff4444)
      .setStrokeStyle(3, 0x3d2817);
    closeBtn.setInteractive(new Phaser.Geom.Rectangle(-30, -30, 60, 60), Phaser.Geom.Rectangle.Contains);
    closeBtn.input!.cursor = 'pointer';
    closeBtn.on('pointerdown', () => this.closeChest());
    this.chestOverlay.add(closeBtn);

    const closeText = this.add.text(850, 120, 'X', {
      fontSize: '32px',
      color: '#fff',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);
    this.chestOverlay.add(closeText);

    // Benched crew section
    const benchBg = this.add.rectangle(512, 300, 760, 180, 0xa0826d, 0.5)
      .setStrokeStyle(3, 0x654321);
    this.chestOverlay.add(benchBg);

    const benchLabel = this.add.text(180, 220, 'BENCHED CREW', {
      fontSize: '24px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);
    this.chestOverlay.add(benchLabel);

    // Display benched crew (crew with slotIndex === null)
    if (this.connection && this.localIdentity) {
      const benchedCrew = Array.from(this.connection.db.crew.iter())
        .filter(c => c.owner && c.owner.isEqual(this.localIdentity) && c.slotIndex === null);

      benchedCrew.forEach((crew, index) => {
        const x = 200 + (index % 8) * 80;
        const y = 270 + Math.floor(index / 8) * 90;

        // Create mini crew card
        const card = this.createMiniCrewCard(crew, x, y);
        this.chestOverlay.add(card);
      });

      if (benchedCrew.length === 0) {
        const emptyText = this.add.text(512, 300, 'No benched crew members', {
          fontSize: '18px',
          color: '#654321',
          fontFamily: 'Georgia, serif',
          fontStyle: 'italic',
        }).setOrigin(0.5);
        this.chestOverlay.add(emptyText);
      }
    }

    // Items section
    const itemsBg = this.add.rectangle(512, 420, 760, 120, 0xa0826d, 0.5)
      .setStrokeStyle(3, 0x654321);
    this.chestOverlay.add(itemsBg);

    const itemsLabel = this.add.text(180, 370, 'ITEM COMPONENTS', {
      fontSize: '24px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);
    this.chestOverlay.add(itemsLabel);

    // Display player's item inventory
    if (this.connection && this.localIdentity) {
      const playerItems = Array.from(this.connection.db.playerItem.iter())
        .filter(item => item.owner && item.owner.isEqual(this.localIdentity));

      if (playerItems.length > 0) {
        playerItems.forEach((item, index) => {
          const x = 200 + (index % 10) * 60;
          const y = 420;

          const itemCard = this.createItemComponentCard(item.component, x, y, item.id);
          this.chestOverlay.add(itemCard);
        });
      } else {
        const emptyText = this.add.text(512, 420, 'No item components in inventory', {
          fontSize: '16px',
          color: '#654321',
          fontFamily: 'Georgia, serif',
          fontStyle: 'italic',
        }).setOrigin(0.5);
        this.chestOverlay.add(emptyText);
      }
    }

    // Stats section
    const statsBg = this.add.rectangle(512, 560, 760, 80, 0xa0826d, 0.5)
      .setStrokeStyle(3, 0x654321);
    this.chestOverlay.add(statsBg);

    const statsLabel = this.add.text(180, 525, 'STATISTICS', {
      fontSize: '24px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);
    this.chestOverlay.add(statsLabel);

    // Get player stats
    if (this.connection && this.localIdentity) {
      const player = Array.from(this.connection.db.player.iter())
        .find(p => p.identity.isEqual(this.localIdentity));

      if (player) {
        const crewCount = Array.from(this.connection.db.crew.iter())
          .filter(c => c.owner && c.owner.isEqual(this.localIdentity)).length;

        const statsText = `Wins: ${player.wins}  |  Losses: ${player.losses}  |  Total Crew: ${crewCount}  |  Ship: ${player.shipType?.tag || 'Raft'}`;
        const stats = this.add.text(512, 560, statsText, {
          fontSize: '18px',
          color: '#3d2817',
          fontFamily: 'Georgia, serif',
          fontStyle: 'bold',
        }).setOrigin(0.5);
        this.chestOverlay.add(stats);
      }
    }

    this.chestOverlay.setVisible(true);
  }

  private createMiniCrewCard(crew: any, x: number, y: number): Phaser.GameObjects.Container {
    const card = this.add.container(x, y);

    // Card background
    const rarityColors: Record<string, number> = {
      Common: 0x87ceeb,
      Rare: 0x4169e1,
      Epic: 0x9370db,
      Legendary: 0xffd700,
    };

    const bg = this.add.rectangle(0, 0, 60, 80, 0xd4b896)
      .setStrokeStyle(3, rarityColors[crew.rarity] || 0x87ceeb);
    card.add(bg);

    // Crew name
    const nameText = this.add.text(0, -30, crew.name.split(' ')[0], {
      fontSize: '10px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
      align: 'center',
      wordWrap: { width: 55 },
    }).setOrigin(0.5);
    card.add(nameText);

    // Level
    const levelText = this.add.text(0, 30, `Lv ${crew.level || 1}`, {
      fontSize: '9px',
      color: '#3d2817',
      fontFamily: 'Georgia, serif',
    }).setOrigin(0.5);
    card.add(levelText);

    // Make clickable to show details
    bg.setInteractive(new Phaser.Geom.Rectangle(-30, -40, 60, 80), Phaser.Geom.Rectangle.Contains);
    bg.input!.cursor = 'pointer';
    bg.on('pointerover', () => {
      bg.setFillStyle(0xe4d4a6);
    });
    bg.on('pointerout', () => {
      bg.setFillStyle(0xd4b896);
    });
    bg.on('pointerdown', () => {
      console.log('Selected crew:', crew.name);
      // TODO: Show detailed crew info or allow placing on board
    });

    return card;
  }

  private closeChest() {
    console.log('📦 Closing treasure chest...');
    this.isChestOpen = false;
    this.chestOverlay.setVisible(false);
  }

  private createItemComponentCard(component: any, x: number, y: number, itemId?: bigint): Phaser.GameObjects.Container {
    const card = this.add.container(x, y);

    // Get component name
    const componentName = this.getItemComponentName(component);

    // Item background with color based on type
    const itemColor = this.getItemComponentColor(component);
    const bg = this.add.rectangle(0, 0, 50, 50, itemColor)
      .setStrokeStyle(2, 0x3d2817);
    card.add(bg);

    // Item abbreviation or icon
    const abbrev = componentName.split(' ').map(w => w[0]).join('').substring(0, 2);
    const text = this.add.text(0, 0, abbrev, {
      fontSize: '16px',
      color: '#fff',
      fontFamily: 'Georgia, serif',
      fontStyle: 'bold',
    }).setOrigin(0.5);
    card.add(text);

    // Make draggable
    card.setSize(50, 50);
    bg.setInteractive(new Phaser.Geom.Rectangle(-25, -25, 50, 50), Phaser.Geom.Rectangle.Contains);
    bg.input!.cursor = 'grab';

    // Store component data on the card for later use
    (card as any).itemComponent = component;
    (card as any).itemId = itemId;

    let tooltipContainer: Phaser.GameObjects.Container | null = null;

    bg.on('pointerover', () => {
      bg.setFillStyle(itemColor, 0.7);

      // Show tooltip with item name
      tooltipContainer = this.add.container(card.x, card.y - 40);
      tooltipContainer.setDepth(2000);

      const tooltipBg = this.add.rectangle(0, 0, componentName.length * 7 + 10, 20, 0x000000, 0.8)
        .setStrokeStyle(1, 0xffd700);
      tooltipContainer.add(tooltipBg);

      const tooltipText = this.add.text(0, 0, componentName, {
        fontSize: '11px',
        color: '#fff',
        fontFamily: 'Georgia, serif',
      }).setOrigin(0.5);
      tooltipContainer.add(tooltipText);
    });

    bg.on('pointerout', () => {
      bg.setFillStyle(itemColor);
      if (tooltipContainer) {
        tooltipContainer.destroy();
        tooltipContainer = null;
      }
    });

    // Enable dragging
    this.input.setDraggable(card);

    card.on('dragstart', (pointer: Phaser.Input.Pointer) => {
      card.setDepth(1500);
      card.setAlpha(0.7);
      bg.input!.cursor = 'grabbing';
      if (tooltipContainer) {
        tooltipContainer.destroy();
        tooltipContainer = null;
      }
    });

    card.on('drag', (pointer: Phaser.Input.Pointer, dragX: number, dragY: number) => {
      card.setPosition(dragX, dragY);
    });

    card.on('dragend', (pointer: Phaser.Input.Pointer) => {
      card.setDepth(0);
      card.setAlpha(1);
      bg.input!.cursor = 'grab';

      // Check if dropped on a crew member
      const droppedOnCrew = this.getCrewAtPosition(pointer.x, pointer.y);

      if (droppedOnCrew && itemId) {
        console.log(`🎒 Equipping ${componentName} to ${droppedOnCrew.name}`);
        this.equipItemToCrew(Number(droppedOnCrew.id), component, Number(itemId));
      }

      // Return to original position
      card.setPosition(x, y);
    });

    return card;
  }

  private getItemComponentName(component: any): string {
    const tag = component?.tag || component;
    const nameMap: Record<string, string> = {
      'Cutlass': 'Cutlass',
      'SniperGoggles': 'Sniper Goggles',
      'ShellDial': 'Shell Dial',
      'ToneDial': 'Tone Dial',
      'SeastoneFragment': 'Seastone Fragment',
      'TidalCloak': 'Tidal Cloak',
      'EnergyDrink': 'Energy Drink',
      'Meat': 'Meat',
    };
    return nameMap[tag] || tag;
  }

  private getItemComponentColor(component: any): number {
    const tag = component?.tag || component;
    const colorMap: Record<string, number> = {
      'Cutlass': 0xff6b6b,           // Red (AD)
      'SniperGoggles': 0xffa500,     // Orange (Crit)
      'ShellDial': 0xffff00,         // Yellow (AS)
      'ToneDial': 0x9370db,          // Purple (AP)
      'SeastoneFragment': 0x708090,  // Gray (Armor)
      'TidalCloak': 0x4169e1,        // Blue (MR)
      'EnergyDrink': 0x00ced1,       // Cyan (Mana)
      'Meat': 0xff69b4,              // Pink (HP)
    };
    return colorMap[tag] || 0x888888;
  }

  private getCompletedItemName(item: any): string {
    const tag = item?.tag || item;
    const nameMap: Record<string, string> = {
      'Yoru': 'Yoru',
      'Kabuto': 'Kabuto',
      'Shusui': 'Shusui',
      'ClimaTact': 'Clima-Tact',
      'ThunderTempo': 'Thunder Tempo',
      'MirageFlower': 'Mirage Flower',
      'AdamWood': 'Adam Wood',
      'SeaKingScale': 'Sea King Scale',
      'ThousandSunnyHull': 'Thousand Sunny Hull',
      'VivrCard': 'Vivre Card',
      'LogPose': 'Log Pose',
      'Poneglyph': 'Poneglyph',
      'GumGumFruit': 'Gum-Gum Fruit',
      'GomuGomuNoMi': 'Gomu Gomu no Mi',
      'HakiMastery': 'Haki Mastery',
    };
    return nameMap[tag] || tag;
  }

  private getCrewAtPosition(x: number, y: number): any | null {
    if (!this.connection || !this.localIdentity) return null;

    // Get all crew on the field
    const allCrew = Array.from(this.connection.db.crew.iter())
      .filter(c => c.owner && c.owner.isEqual(this.localIdentity) && c.slotIndex !== null);

    // Check each crew's position
    for (const crew of allCrew) {
      const slotIndex = Number(crew.slotIndex);
      const gridX = slotIndex % 5;
      const gridY = Math.floor(slotIndex / 5);

      const screenPos = this.gridToScreen(gridX, gridY);

      // Check if click is within crew bounds (50x50 square)
      if (Math.abs(x - screenPos.x) <= 25 && Math.abs(y - screenPos.y) <= 25) {
        return crew;
      }
    }

    return null;
  }

  private equipItemToCrew(crewId: number, component: any, itemId: number): void {
    if (!this.connection) {
      console.error('No connection available');
      return;
    }

    try {
      // Call the reducer to equip the item
      this.connection.reducers.equipItemToCrew(BigInt(crewId), component);
      console.log(`✅ Successfully equipped item to crew ${crewId}`);

      // Close and reopen chest to refresh display
      this.closeChest();
      setTimeout(() => this.openChest(), 100);
    } catch (error) {
      console.error('Failed to equip item:', error);
    }
  }
}
