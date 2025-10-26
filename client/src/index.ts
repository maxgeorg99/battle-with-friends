/**
 * Battle with Friends - Client Library
 * Central export file for easy imports
 */

// Theme
export { theme, getRarityColor } from './theme';
export type { Theme } from './theme';

// UI Components
export {
  Button,
  Card,
  Modal,
  Input,
  Badge,
  Toast,
  useToast,
} from './components/ui';

export type {
  ButtonProps,
  CardProps,
  ModalProps,
  InputProps,
  BadgeProps,
  ToastProps,
} from './components/ui';

// Game Components
export {
  CrewCard,
  ItemSlot,
} from './components/game';

export type {
  CrewCardProps,
  CrewMember,
  ItemSlotProps,
  Item,
} from './components/game';

// Phaser UI Utilities
export {
  UIFactory,
  PhaserTheme,
  getRarityCSSColor,
} from './game/ui';

// Showcase (for development)
export { ComponentShowcase } from './components/ComponentShowcase';
