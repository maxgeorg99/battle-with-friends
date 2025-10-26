/**
 * Design System / Theme
 * Single source of truth for colors, fonts, spacing, and styling
 */

export const theme = {
  colors: {
    // Primary palette
    primary: '#ffd700',       // Gold - buttons, highlights
    secondary: '#4a90e2',     // Ocean blue - accents
    danger: '#ff4444',        // Red - warnings, delete
    success: '#4caf50',       // Green - confirmations

    // Background colors
    background: {
      dark: '#16213e',        // Dark navy
      darker: '#0f3460',      // Deeper navy
      darkest: '#1a1a2e',     // Almost black
    },

    // Game-specific colors
    wood: '#d4b896',          // Parchment/wood texture
    woodDark: '#654321',      // Dark brown
    woodLight: '#a0826d',     // Lighter wood
    sea: '#4a90e2',           // Ocean

    // Rarity colors (for crew/items)
    rarity: {
      common: '#87ceeb',      // Sky blue
      rare: '#4169e1',        // Royal blue
      epic: '#9370db',        // Purple
      legendary: '#ffd700',   // Gold
    },

    // Text colors
    text: {
      primary: '#ffffff',     // White
      secondary: '#b0b0b0',   // Light gray
      muted: '#808080',       // Gray
      disabled: '#4a4a4a',    // Dark gray
    },

    // UI states
    hover: 'rgba(255, 215, 0, 0.1)',  // Gold with 10% opacity
    active: 'rgba(255, 215, 0, 0.2)', // Gold with 20% opacity
    disabled: 'rgba(255, 255, 255, 0.1)',

    // Overlays
    overlay: 'rgba(0, 0, 0, 0.7)',    // Semi-transparent black
    overlayLight: 'rgba(0, 0, 0, 0.5)',
  },

  fonts: {
    heading: 'Georgia, "Times New Roman", serif',
    body: 'Arial, Helvetica, sans-serif',
    mono: '"Courier New", Courier, monospace',
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
    xs: '4px',
    sm: '8px',
    md: '16px',
    lg: '24px',
    xl: '32px',
    xxl: '48px',
  },

  borderRadius: {
    sm: '4px',
    md: '8px',
    lg: '16px',
    full: '9999px',
  },

  shadows: {
    sm: '0 1px 3px rgba(0, 0, 0, 0.3)',
    md: '0 4px 6px rgba(0, 0, 0, 0.4)',
    lg: '0 10px 20px rgba(0, 0, 0, 0.5)',
    glow: '0 0 10px rgba(255, 215, 0, 0.5)', // Gold glow
  },

  transitions: {
    fast: '150ms ease-in-out',
    normal: '250ms ease-in-out',
    slow: '400ms ease-in-out',
  },

  zIndex: {
    base: 0,
    dropdown: 1000,
    modal: 2000,
    tooltip: 3000,
    notification: 4000,
  },
} as const;

// Helper function to get rarity color
export function getRarityColor(rarity: 'common' | 'rare' | 'epic' | 'legendary'): string {
  return theme.colors.rarity[rarity];
}

// Type export for TypeScript autocomplete
export type Theme = typeof theme;
