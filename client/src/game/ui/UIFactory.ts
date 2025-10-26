/**
 * UI Factory for creating consistent Phaser UI elements
 */
import Phaser from 'phaser';
import { PhaserTheme, getRarityCSSColor } from './Theme';

export class UIFactory {
  /**
   * Create a styled button
   */
  static createButton(
    scene: Phaser.Scene,
    x: number,
    y: number,
    text: string,
    onClick: () => void,
    options: {
      width?: number;
      height?: number;
      backgroundColor?: number;
      textColor?: string;
      fontSize?: string;
    } = {}
  ): Phaser.GameObjects.Container {
    const {
      width = 150,
      height = 40,
      backgroundColor = PhaserTheme.colors.primary,
      textColor = PhaserTheme.cssColors.darkGray,
      fontSize = PhaserTheme.fontSizes.md,
    } = options;

    const container = scene.add.container(x, y);

    // Background
    const bg = scene.add.rectangle(0, 0, width, height, backgroundColor, 1);
    bg.setStrokeStyle(2, backgroundColor, 1);
    bg.setInteractive({ useHandCursor: true });

    // Text
    const label = scene.add.text(0, 0, text, {
      fontFamily: PhaserTheme.fonts.body,
      fontSize,
      color: textColor,
      fontStyle: 'bold',
    });
    label.setOrigin(0.5);

    container.add([bg, label]);

    // Hover effects
    bg.on('pointerover', () => {
      bg.setFillStyle(backgroundColor, 0.8);
      container.setScale(1.05);
    });

    bg.on('pointerout', () => {
      bg.setFillStyle(backgroundColor, 1);
      container.setScale(1);
    });

    bg.on('pointerdown', () => {
      container.setScale(0.95);
    });

    bg.on('pointerup', () => {
      container.setScale(1.05);
      onClick();
    });

    return container;
  }

  /**
   * Create a card container with border and background
   */
  static createCard(
    scene: Phaser.Scene,
    x: number,
    y: number,
    width: number,
    height: number,
    options: {
      backgroundColor?: number;
      borderColor?: number;
      borderWidth?: number;
      borderRadius?: number;
      interactive?: boolean;
      onClick?: () => void;
    } = {}
  ): Phaser.GameObjects.Graphics {
    const {
      backgroundColor = PhaserTheme.colors.background.dark,
      borderColor = PhaserTheme.colors.background.darker,
      borderWidth = 2,
      borderRadius = PhaserTheme.borderRadius.lg,
      interactive = false,
      onClick,
    } = options;

    const graphics = scene.add.graphics();
    graphics.setPosition(x, y);

    // Draw rounded rectangle
    graphics.fillStyle(backgroundColor, 1);
    graphics.lineStyle(borderWidth, borderColor, 1);
    graphics.fillRoundedRect(0, 0, width, height, borderRadius);
    graphics.strokeRoundedRect(0, 0, width, height, borderRadius);

    if (interactive && onClick) {
      const hitArea = new Phaser.Geom.Rectangle(0, 0, width, height);
      graphics.setInteractive(hitArea, Phaser.Geom.Rectangle.Contains);
      graphics.on('pointerdown', onClick);
    }

    return graphics;
  }

  /**
   * Create a text label with consistent styling
   */
  static createText(
    scene: Phaser.Scene,
    x: number,
    y: number,
    text: string,
    options: {
      fontSize?: string;
      color?: string;
      fontFamily?: string;
      fontStyle?: string;
      align?: 'left' | 'center' | 'right';
      origin?: { x: number; y: number };
    } = {}
  ): Phaser.GameObjects.Text {
    const {
      fontSize = PhaserTheme.fontSizes.md,
      color = PhaserTheme.cssColors.white,
      fontFamily = PhaserTheme.fonts.body,
      fontStyle = 'normal',
      align = 'left',
      origin = { x: 0, y: 0 },
    } = options;

    const textObj = scene.add.text(x, y, text, {
      fontFamily,
      fontSize,
      color,
      fontStyle,
      align,
    });
    textObj.setOrigin(origin.x, origin.y);

    return textObj;
  }

  /**
   * Create a badge (small pill-shaped label)
   */
  static createBadge(
    scene: Phaser.Scene,
    x: number,
    y: number,
    text: string,
    rarity?: 'common' | 'rare' | 'epic' | 'legendary'
  ): Phaser.GameObjects.Container {
    const container = scene.add.container(x, y);

    const backgroundColor = rarity
      ? PhaserTheme.colors.rarity[rarity]
      : PhaserTheme.colors.primary;

    const textColor = rarity === 'common' || rarity === 'legendary'
      ? PhaserTheme.cssColors.darkGray
      : PhaserTheme.cssColors.white;

    // Calculate dimensions
    const padding = PhaserTheme.spacing.xs;
    const tempText = scene.add.text(0, 0, text, {
      fontFamily: PhaserTheme.fonts.body,
      fontSize: PhaserTheme.fontSizes.xs,
      fontStyle: 'bold',
    });
    const textWidth = tempText.width;
    const textHeight = tempText.height;
    tempText.destroy();

    // Background
    const bg = scene.add.graphics();
    const bgWidth = textWidth + padding * 2;
    const bgHeight = textHeight + padding;
    bg.fillStyle(backgroundColor, 1);
    bg.fillRoundedRect(-bgWidth / 2, -bgHeight / 2, bgWidth, bgHeight, bgHeight / 2);

    // Text
    const label = scene.add.text(0, 0, text, {
      fontFamily: PhaserTheme.fonts.body,
      fontSize: PhaserTheme.fontSizes.xs,
      color: textColor,
      fontStyle: 'bold',
    });
    label.setOrigin(0.5);

    container.add([bg, label]);

    return container;
  }

  /**
   * Create a progress bar
   */
  static createProgressBar(
    scene: Phaser.Scene,
    x: number,
    y: number,
    width: number,
    height: number,
    progress: number, // 0 to 1
    options: {
      backgroundColor?: number;
      fillColor?: number;
      borderColor?: number;
    } = {}
  ): Phaser.GameObjects.Container {
    const {
      backgroundColor = PhaserTheme.colors.background.darker,
      fillColor = PhaserTheme.colors.success,
      borderColor = PhaserTheme.colors.background.darkest,
    } = options;

    const container = scene.add.container(x, y);

    // Background
    const bg = scene.add.rectangle(0, 0, width, height, backgroundColor);
    bg.setStrokeStyle(2, borderColor);
    bg.setOrigin(0, 0.5);

    // Fill
    const fillWidth = Math.max(0, Math.min(1, progress)) * width;
    const fill = scene.add.rectangle(0, 0, fillWidth, height - 4, fillColor);
    fill.setOrigin(0, 0.5);

    container.add([bg, fill]);

    // Method to update progress
    (container as any).updateProgress = (newProgress: number) => {
      const newFillWidth = Math.max(0, Math.min(1, newProgress)) * width;
      fill.width = newFillWidth;
    };

    return container;
  }

  /**
   * Create a tooltip container
   */
  static createTooltip(
    scene: Phaser.Scene,
    x: number,
    y: number,
    text: string,
    maxWidth: number = 200
  ): Phaser.GameObjects.Container {
    const container = scene.add.container(x, y);
    container.setDepth(10000); // Always on top

    const padding = PhaserTheme.spacing.sm;

    // Text
    const label = scene.add.text(0, 0, text, {
      fontFamily: PhaserTheme.fonts.body,
      fontSize: PhaserTheme.fontSizes.sm,
      color: PhaserTheme.cssColors.white,
      wordWrap: { width: maxWidth - padding * 2 },
    });
    label.setOrigin(0.5);

    // Background
    const bg = scene.add.graphics();
    const bgWidth = label.width + padding * 2;
    const bgHeight = label.height + padding * 2;
    bg.fillStyle(PhaserTheme.colors.background.darkest, 0.95);
    bg.lineStyle(1, PhaserTheme.colors.primary, 1);
    bg.fillRoundedRect(-bgWidth / 2, -bgHeight / 2, bgWidth, bgHeight, 4);
    bg.strokeRoundedRect(-bgWidth / 2, -bgHeight / 2, bgWidth, bgHeight, 4);

    container.add([bg, label]);

    return container;
  }
}
