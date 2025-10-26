# Battle with Friends - Component Library

A lean, custom-built component library for the Battle with Friends game. No unnecessary dependencies, just solid React components and Phaser utilities.

## Philosophy

- **Lightweight**: No heavy frameworks, just what we need
- **Consistent**: All components follow the same design system
- **Game-focused**: Built specifically for a One Piece-themed auto-battler
- **Type-safe**: Full TypeScript support

## Theme System

Located at `src/theme.ts`, provides:
- Color palette (primary, rarity colors, backgrounds, etc.)
- Typography (fonts, sizes)
- Spacing system
- Border radius, shadows, transitions
- Z-index layers

```typescript
import { theme, getRarityColor } from '../theme';
```

## UI Components (`/components/ui`)

### Button
```tsx
import { Button } from './components/ui';

<Button variant="primary" size="md" onClick={handleClick}>
  Click Me
</Button>
```

Variants: `primary` | `secondary` | `danger`
Sizes: `sm` | `md` | `lg`

### Card
```tsx
import { Card } from './components/ui';

<Card title="Card Title" hoverable onClick={handleClick}>
  Card content here
</Card>
```

### Modal
```tsx
import { Modal } from './components/ui';

<Modal isOpen={isOpen} onClose={handleClose} title="Modal Title">
  Modal content here
</Modal>
```

Features: ESC to close, click outside to close, scrollable content

### Input
```tsx
import { Input } from './components/ui';

<Input
  label="Username"
  value={value}
  onChange={setValue}
  placeholder="Enter username..."
  error="This field is required"
/>
```

### Badge
```tsx
import { Badge } from './components/ui';

<Badge variant="legendary" size="sm">
  Level 10
</Badge>
```

Variants: `primary` | `secondary` | `success` | `danger` | `common` | `rare` | `epic` | `legendary`

### Toast
```tsx
import { Toast, useToast } from './components/ui';

const { toast, showToast, hideToast } = useToast();

// Show toast
showToast('Battle started!', 'success');

// Render toast
<Toast
  message={toast.message}
  type={toast.type}
  isVisible={toast.isVisible}
  onClose={hideToast}
/>
```

## Game Components (`/components/game`)

### CrewCard
```tsx
import { CrewCard } from './components/game';

const crew: CrewMember = {
  id: '1',
  name: 'Luffy',
  rarity: 'legendary',
  level: 10,
  maxHp: 500,
  attack: 120,
  defense: 80,
  traits: ['Rubber', 'Captain'],
};

<CrewCard crew={crew} onClick={handleClick} compact={false} />
```

### ItemSlot
```tsx
import { ItemSlot } from './components/game';

const item: Item = {
  id: 'item1',
  name: 'Straw Hat',
  rarity: 'legendary',
  stats: { hp: 50, attack: 10 },
};

<ItemSlot item={item} size={64} onClick={handleClick} />
```

## Phaser UI Utilities (`/game/ui`)

### Theme
```typescript
import { PhaserTheme, getRarityColor } from '../game/ui';

// Use in Phaser
const color = PhaserTheme.colors.primary; // 0xffd700
const rarityColor = getRarityColor('legendary'); // 0xffd700
```

### UIFactory
Helper class for creating consistent Phaser UI elements:

```typescript
import { UIFactory } from '../game/ui';

// Create button
const button = UIFactory.createButton(
  scene,
  x, y,
  'Click Me',
  () => console.log('clicked'),
  { width: 150, height: 40 }
);

// Create card
const card = UIFactory.createCard(scene, x, y, width, height, {
  backgroundColor: PhaserTheme.colors.background.dark,
  borderColor: PhaserTheme.colors.primary,
});

// Create text
const text = UIFactory.createText(scene, x, y, 'Hello World', {
  fontSize: PhaserTheme.fontSizes.lg,
  color: PhaserTheme.cssColors.primary,
});

// Create badge
const badge = UIFactory.createBadge(scene, x, y, 'Epic', 'epic');

// Create progress bar
const progressBar = UIFactory.createProgressBar(
  scene, x, y, width, height, 0.75
);
// Update progress
(progressBar as any).updateProgress(0.5);

// Create tooltip
const tooltip = UIFactory.createTooltip(scene, x, y, 'Tooltip text');
```

## Component Showcase

To see all components in action, import and render `ComponentShowcase`:

```tsx
import { ComponentShowcase } from './components/ComponentShowcase';

<ComponentShowcase />
```

## Adding New Components

1. Create component in appropriate directory (`ui` or `game`)
2. Follow existing patterns (TypeScript, props interface, inline styles)
3. Use theme system for all colors, spacing, fonts
4. Export from index file
5. Add to showcase for testing

## Design Principles

- **Inline styles**: Simple, no CSS files to manage
- **TypeScript**: Full type safety
- **Hover effects**: All interactive elements have hover states
- **Consistent spacing**: Use `theme.spacing.*`
- **Accessibility**: Keyboard navigation, semantic HTML
- **Performance**: Lightweight, no heavy dependencies
