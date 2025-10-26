import React from 'react';
import { theme, getRarityColor } from '../../theme';

export interface Item {
  id: string;
  name: string;
  rarity: 'common' | 'rare' | 'epic' | 'legendary';
  iconUrl?: string;
  stats?: {
    hp?: number;
    attack?: number;
    defense?: number;
    ap?: number;
    mr?: number;
  };
}

export interface ItemSlotProps {
  item?: Item | null;
  size?: number;
  onClick?: () => void;
  disabled?: boolean;
  showTooltip?: boolean;
}

export const ItemSlot: React.FC<ItemSlotProps> = ({
  item,
  size = 64,
  onClick,
  disabled = false,
  showTooltip = true,
}) => {
  const [isHovered, setIsHovered] = React.useState(false);

  const borderColor = item ? getRarityColor(item.rarity) : theme.colors.background.darker;

  const slotStyle: React.CSSProperties = {
    width: `${size}px`,
    height: `${size}px`,
    background: theme.colors.background.darker,
    border: `2px solid ${borderColor}`,
    borderRadius: theme.borderRadius.md,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    cursor: disabled ? 'not-allowed' : onClick ? 'pointer' : 'default',
    transition: theme.transitions.normal,
    opacity: disabled ? 0.5 : 1,
    position: 'relative',
    boxShadow: item ? theme.shadows.md : 'none',
    ...(isHovered && !disabled && { transform: 'scale(1.05)', boxShadow: theme.shadows.lg }),
  };

  const emptyTextStyle: React.CSSProperties = {
    color: theme.colors.text.muted,
    fontSize: theme.fontSizes.xs,
  };

  const iconStyle: React.CSSProperties = {
    width: '90%',
    height: '90%',
    objectFit: 'contain',
  };

  const tooltipStyle: React.CSSProperties = {
    position: 'absolute',
    bottom: '100%',
    left: '50%',
    transform: 'translateX(-50%)',
    marginBottom: theme.spacing.sm,
    background: theme.colors.background.darkest,
    border: `1px solid ${theme.colors.primary}`,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.sm,
    minWidth: '150px',
    zIndex: theme.zIndex.tooltip,
    pointerEvents: 'none',
    boxShadow: theme.shadows.lg,
  };

  const tooltipNameStyle: React.CSSProperties = {
    fontFamily: theme.fonts.body,
    fontSize: theme.fontSizes.sm,
    color: borderColor,
    fontWeight: 'bold',
    marginBottom: theme.spacing.xs,
  };

  const tooltipStatsStyle: React.CSSProperties = {
    fontFamily: theme.fonts.body,
    fontSize: theme.fontSizes.xs,
    color: theme.colors.text.secondary,
    lineHeight: 1.5,
  };

  return (
    <div
      style={slotStyle}
      onClick={disabled ? undefined : onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      {item ? (
        <>
          {item.iconUrl ? (
            <img src={item.iconUrl} alt={item.name} style={iconStyle} />
          ) : (
            <span style={{ fontSize: theme.fontSizes.lg }}>📦</span>
          )}

          {/* Tooltip */}
          {isHovered && showTooltip && (
            <div style={tooltipStyle}>
              <div style={tooltipNameStyle}>{item.name}</div>
              {item.stats && (
                <div style={tooltipStatsStyle}>
                  {item.stats.hp && <div>+{item.stats.hp} HP</div>}
                  {item.stats.attack && <div>+{item.stats.attack} ATK</div>}
                  {item.stats.defense && <div>+{item.stats.defense} DEF</div>}
                  {item.stats.ap && <div>+{item.stats.ap} AP</div>}
                  {item.stats.mr && <div>+{item.stats.mr} MR</div>}
                </div>
              )}
            </div>
          )}
        </>
      ) : (
        <span style={emptyTextStyle}>Empty</span>
      )}
    </div>
  );
};
