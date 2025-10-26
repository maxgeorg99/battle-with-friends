import React from 'react';
import { theme, getRarityColor } from '../../theme';
import { Card } from '../ui/Card';
import { Badge } from '../ui/Badge';

export interface CrewMember {
  id: string;
  name: string;
  rarity: 'common' | 'rare' | 'epic' | 'legendary';
  level: number;
  maxHp: number;
  attack: number;
  defense: number;
  traits?: string[];
  imageUrl?: string;
}

export interface CrewCardProps {
  crew: CrewMember;
  onClick?: () => void;
  compact?: boolean;
}

export const CrewCard: React.FC<CrewCardProps> = ({ crew, onClick, compact = false }) => {
  const rarityColor = getRarityColor(crew.rarity);

  const containerStyle: React.CSSProperties = {
    position: 'relative',
    border: `3px solid ${rarityColor}`,
    borderRadius: theme.borderRadius.lg,
    overflow: 'hidden',
  };

  const headerStyle: React.CSSProperties = {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.sm,
  };

  const nameStyle: React.CSSProperties = {
    fontFamily: theme.fonts.heading,
    fontSize: compact ? theme.fontSizes.md : theme.fontSizes.lg,
    color: theme.colors.text.primary,
    margin: 0,
  };

  const statsContainerStyle: React.CSSProperties = {
    display: 'grid',
    gridTemplateColumns: 'repeat(3, 1fr)',
    gap: theme.spacing.sm,
    marginTop: theme.spacing.sm,
  };

  const statStyle: React.CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    gap: theme.spacing.xs,
  };

  const statLabelStyle: React.CSSProperties = {
    fontFamily: theme.fonts.body,
    fontSize: theme.fontSizes.xs,
    color: theme.colors.text.secondary,
    textTransform: 'uppercase',
  };

  const statValueStyle: React.CSSProperties = {
    fontFamily: theme.fonts.body,
    fontSize: theme.fontSizes.md,
    color: theme.colors.text.primary,
    fontWeight: 'bold',
  };

  const traitsContainerStyle: React.CSSProperties = {
    display: 'flex',
    flexWrap: 'wrap',
    gap: theme.spacing.xs,
    marginTop: theme.spacing.sm,
  };

  const imageContainerStyle: React.CSSProperties = {
    width: '100%',
    height: compact ? '80px' : '120px',
    background: theme.colors.background.darker,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    overflow: 'hidden',
  };

  const imageStyle: React.CSSProperties = {
    width: '100%',
    height: '100%',
    objectFit: 'cover',
  };

  return (
    <Card onClick={onClick} hoverable={!!onClick} padding={compact ? 'sm' : 'md'} style={containerStyle}>
      {/* Image placeholder */}
      {!compact && (
        <div style={imageContainerStyle}>
          {crew.imageUrl ? (
            <img src={crew.imageUrl} alt={crew.name} style={imageStyle} />
          ) : (
            <span style={{ color: theme.colors.text.muted, fontSize: theme.fontSizes.sm }}>
              No Image
            </span>
          )}
        </div>
      )}

      {/* Header */}
      <div style={headerStyle}>
        <h3 style={nameStyle}>{crew.name}</h3>
        <Badge variant={crew.rarity} size="sm">
          Lv {crew.level}
        </Badge>
      </div>

      {/* Stats */}
      {!compact && (
        <div style={statsContainerStyle}>
          <div style={statStyle}>
            <span style={statLabelStyle}>HP</span>
            <span style={statValueStyle}>{crew.maxHp}</span>
          </div>
          <div style={statStyle}>
            <span style={statLabelStyle}>ATK</span>
            <span style={statValueStyle}>{crew.attack}</span>
          </div>
          <div style={statStyle}>
            <span style={statLabelStyle}>DEF</span>
            <span style={statValueStyle}>{crew.defense}</span>
          </div>
        </div>
      )}

      {/* Traits */}
      {crew.traits && crew.traits.length > 0 && !compact && (
        <div style={traitsContainerStyle}>
          {crew.traits.map((trait, index) => (
            <Badge key={index} variant="secondary" size="sm">
              {trait}
            </Badge>
          ))}
        </div>
      )}
    </Card>
  );
};
