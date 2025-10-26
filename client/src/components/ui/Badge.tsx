import React from 'react';
import { theme } from '../../theme';

export interface BadgeProps {
  children: React.ReactNode;
  variant?: 'primary' | 'secondary' | 'success' | 'danger' | 'common' | 'rare' | 'epic' | 'legendary';
  size?: 'sm' | 'md';
}

export const Badge: React.FC<BadgeProps> = ({
  children,
  variant = 'primary',
  size = 'md',
}) => {
  const variantStyles: Record<string, React.CSSProperties> = {
    primary: {
      background: theme.colors.primary,
      color: theme.colors.background.darkest,
    },
    secondary: {
      background: theme.colors.secondary,
      color: theme.colors.text.primary,
    },
    success: {
      background: theme.colors.success,
      color: theme.colors.text.primary,
    },
    danger: {
      background: theme.colors.danger,
      color: theme.colors.text.primary,
    },
    common: {
      background: theme.colors.rarity.common,
      color: theme.colors.background.darkest,
    },
    rare: {
      background: theme.colors.rarity.rare,
      color: theme.colors.text.primary,
    },
    epic: {
      background: theme.colors.rarity.epic,
      color: theme.colors.text.primary,
    },
    legendary: {
      background: theme.colors.rarity.legendary,
      color: theme.colors.background.darkest,
    },
  };

  const sizeStyles: Record<string, React.CSSProperties> = {
    sm: {
      padding: `2px ${theme.spacing.xs}`,
      fontSize: theme.fontSizes.xs,
    },
    md: {
      padding: `${theme.spacing.xs} ${theme.spacing.sm}`,
      fontSize: theme.fontSizes.sm,
    },
  };

  const baseStyle: React.CSSProperties = {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: theme.borderRadius.full,
    fontFamily: theme.fonts.body,
    fontWeight: '600',
    whiteSpace: 'nowrap',
    boxShadow: theme.shadows.sm,
  };

  const style: React.CSSProperties = {
    ...baseStyle,
    ...variantStyles[variant],
    ...sizeStyles[size],
  };

  return <span style={style}>{children}</span>;
};
