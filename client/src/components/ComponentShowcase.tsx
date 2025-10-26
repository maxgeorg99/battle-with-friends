import React from 'react';
import { theme } from '../theme';
import { Button, Card, Modal, Input, Badge, Toast, useToast } from './ui';
import { CrewCard, ItemSlot } from './game';
import type { CrewMember, Item } from './game';

/**
 * Component Showcase
 * Demo page showing all available UI components
 * Useful for development and testing
 */
export const ComponentShowcase: React.FC = () => {
  const [isModalOpen, setIsModalOpen] = React.useState(false);
  const [inputValue, setInputValue] = React.useState('');
  const { toast, showToast, hideToast } = useToast();

  // Example data
  const exampleCrew: CrewMember = {
    id: '1',
    name: 'Monkey D. Luffy',
    rarity: 'legendary',
    level: 10,
    maxHp: 500,
    attack: 120,
    defense: 80,
    traits: ['Rubber', 'Captain', 'D.'],
  };

  const exampleItem: Item = {
    id: 'item1',
    name: 'Straw Hat',
    rarity: 'legendary',
    stats: {
      hp: 50,
      attack: 10,
      defense: 5,
    },
  };

  const pageStyle: React.CSSProperties = {
    background: theme.colors.background.darkest,
    minHeight: '100vh',
    padding: theme.spacing.xl,
    color: theme.colors.text.primary,
    fontFamily: theme.fonts.body,
  };

  const sectionStyle: React.CSSProperties = {
    marginBottom: theme.spacing.xxl,
  };

  const titleStyle: React.CSSProperties = {
    fontFamily: theme.fonts.heading,
    fontSize: theme.fontSizes.xxl,
    color: theme.colors.primary,
    marginBottom: theme.spacing.lg,
  };

  const subtitleStyle: React.CSSProperties = {
    fontFamily: theme.fonts.heading,
    fontSize: theme.fontSizes.xl,
    color: theme.colors.text.primary,
    marginBottom: theme.spacing.md,
  };

  const gridStyle: React.CSSProperties = {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
    gap: theme.spacing.md,
    marginBottom: theme.spacing.md,
  };

  const flexStyle: React.CSSProperties = {
    display: 'flex',
    gap: theme.spacing.md,
    flexWrap: 'wrap',
    marginBottom: theme.spacing.md,
  };

  return (
    <div style={pageStyle}>
      <h1 style={titleStyle}>Battle with Friends - Component Library</h1>

      {/* Buttons */}
      <section style={sectionStyle}>
        <h2 style={subtitleStyle}>Buttons</h2>
        <div style={flexStyle}>
          <Button variant="primary" onClick={() => showToast('Primary clicked!', 'success')}>
            Primary Button
          </Button>
          <Button variant="secondary" onClick={() => showToast('Secondary clicked!', 'info')}>
            Secondary Button
          </Button>
          <Button variant="danger" onClick={() => showToast('Danger clicked!', 'error')}>
            Danger Button
          </Button>
          <Button variant="primary" size="sm">
            Small
          </Button>
          <Button variant="primary" size="lg">
            Large
          </Button>
          <Button variant="primary" disabled>
            Disabled
          </Button>
        </div>
      </section>

      {/* Badges */}
      <section style={sectionStyle}>
        <h2 style={subtitleStyle}>Badges</h2>
        <div style={flexStyle}>
          <Badge variant="primary">Primary</Badge>
          <Badge variant="secondary">Secondary</Badge>
          <Badge variant="success">Success</Badge>
          <Badge variant="danger">Danger</Badge>
          <Badge variant="common">Common</Badge>
          <Badge variant="rare">Rare</Badge>
          <Badge variant="epic">Epic</Badge>
          <Badge variant="legendary">Legendary</Badge>
          <Badge size="sm">Small</Badge>
          <Badge size="md">Medium</Badge>
        </div>
      </section>

      {/* Cards */}
      <section style={sectionStyle}>
        <h2 style={subtitleStyle}>Cards</h2>
        <div style={gridStyle}>
          <Card title="Basic Card">
            <p style={{ margin: 0, color: theme.colors.text.secondary }}>
              This is a basic card with a title and some content.
            </p>
          </Card>
          <Card hoverable>
            <p style={{ margin: 0, color: theme.colors.text.secondary }}>
              This card is hoverable.
            </p>
          </Card>
          <Card onClick={() => showToast('Card clicked!', 'info')}>
            <p style={{ margin: 0, color: theme.colors.text.secondary }}>
              This card is clickable.
            </p>
          </Card>
        </div>
      </section>

      {/* Input */}
      <section style={sectionStyle}>
        <h2 style={subtitleStyle}>Input</h2>
        <div style={{ maxWidth: '400px' }}>
          <Input
            label="Username"
            placeholder="Enter username..."
            value={inputValue}
            onChange={setInputValue}
            fullWidth
          />
          <div style={{ marginTop: theme.spacing.md }}>
            <Input
              label="Password"
              type="password"
              placeholder="Enter password..."
              value=""
              onChange={() => {}}
              fullWidth
            />
          </div>
          <div style={{ marginTop: theme.spacing.md }}>
            <Input
              label="Error Example"
              placeholder="This has an error"
              value=""
              onChange={() => {}}
              error="This field is required"
              fullWidth
            />
          </div>
        </div>
      </section>

      {/* Modal */}
      <section style={sectionStyle}>
        <h2 style={subtitleStyle}>Modal</h2>
        <Button onClick={() => setIsModalOpen(true)}>Open Modal</Button>
        <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Example Modal">
          <p style={{ color: theme.colors.text.secondary, marginBottom: theme.spacing.md }}>
            This is a modal dialog. Press ESC or click outside to close.
          </p>
          <div style={{ display: 'flex', gap: theme.spacing.sm, justifyContent: 'flex-end' }}>
            <Button variant="secondary" onClick={() => setIsModalOpen(false)}>
              Cancel
            </Button>
            <Button
              variant="primary"
              onClick={() => {
                setIsModalOpen(false);
                showToast('Action confirmed!', 'success');
              }}
            >
              Confirm
            </Button>
          </div>
        </Modal>
      </section>

      {/* Game Components */}
      <section style={sectionStyle}>
        <h2 style={subtitleStyle}>Game Components</h2>

        <h3 style={{ ...subtitleStyle, fontSize: theme.fontSizes.lg }}>Crew Card</h3>
        <div style={{ maxWidth: '300px', marginBottom: theme.spacing.lg }}>
          <CrewCard crew={exampleCrew} onClick={() => showToast('Crew card clicked!', 'info')} />
        </div>

        <h3 style={{ ...subtitleStyle, fontSize: theme.fontSizes.lg }}>Item Slots</h3>
        <div style={flexStyle}>
          <ItemSlot item={exampleItem} onClick={() => showToast('Item clicked!', 'info')} />
          <ItemSlot item={null} onClick={() => showToast('Empty slot clicked!', 'info')} />
          <ItemSlot item={exampleItem} size={48} />
          <ItemSlot item={exampleItem} disabled />
        </div>
      </section>

      {/* Toast Notifications */}
      <section style={sectionStyle}>
        <h2 style={subtitleStyle}>Toast Notifications</h2>
        <p style={{ color: theme.colors.text.secondary, marginBottom: theme.spacing.md }}>
          Click any button above to see toast notifications in action!
        </p>
      </section>

      {/* Toast Component */}
      <Toast
        message={toast.message}
        type={toast.type}
        isVisible={toast.isVisible}
        onClose={hideToast}
      />
    </div>
  );
};
