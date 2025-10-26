/**
 * Phaser UI Theme
 * Provides consistent styling for Phaser game objects
 */

export const PhaserTheme = {
  colors: {
    // Convert hex to numbers for Phaser
    primary: 0xffd700,       // Gold
    secondary: 0x4a90e2,     // Ocean blue
    danger: 0xff4444,        // Red
    success: 0x4caf50,       // Green

    background: {
      dark: 0x16213e,        // Dark navy
      darker: 0x0f3460,      // Deeper navy
      darkest: 0x1a1a2e,     // Almost black
    },

    wood: 0xd4b896,          // Parchment/wood
    woodDark: 0x654321,      // Dark brown
    woodLight: 0xa0826d,     // Lighter wood
    sea: 0x4a90e2,           // Ocean

    rarity: {
      common: 0x87ceeb,      // Sky blue
      rare: 0x4169e1,        // Royal blue
      epic: 0x9370db,        // Purple
      legendary: 0xffd700,   // Gold
    },

    text: {
      white: 0xffffff,
      lightGray: 0xb0b0b0,
      gray: 0x808080,
      darkGray: 0x4a4a4a,
    },
  },

  // CSS color strings (for text)
  cssColors: {
    primary: '#ffd700',
    secondary: '#4a90e2',
    danger: '#ff4444',
    success: '#4caf50',
    white: '#ffffff',
    lightGray: '#b0b0b0',
    gray: '#808080',
    darkGray: '#4a4a4a',

    rarity: {
      common: '#87ceeb',
      rare: '#4169e1',
      epic: '#9370db',
      legendary: '#ffd700',
    },
  },

  fonts: {
    heading: 'Georgia',
    body: 'Arial',
  },

  fontSizes: {
    xs: '12px',
    sm: '14px',
    md: '16px',
    lg: '18px',
    xl: '24px',
    xxl: '32px',
  },

  spacing: {
    xs: 4,
    sm: 8,
    md: 16,
    lg: 24,
    xl: 32,
    xxl: 48,
  },

  borderRadius: {
    sm: 4,
    md: 8,
    lg: 16,
  },
} as const;

// Helper to get rarity color
export function getRarityColor(rarity: 'common' | 'rare' | 'epic' | 'legendary'): number {
  return PhaserTheme.colors.rarity[rarity];
}

// Helper to get rarity CSS color
export function getRarityCSSColor(rarity: 'common' | 'rare' | 'epic' | 'legendary'): string {
  return PhaserTheme.cssColors.rarity[rarity];
}
