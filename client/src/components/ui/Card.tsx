import React from 'react';
import { theme } from '../../theme';

export interface CardProps {
  children: React.ReactNode;
  title?: string;
  onClick?: () => void;
  hoverable?: boolean;
  padding?: 'sm' | 'md' | 'lg';
  style?: React.CSSProperties;
}

export const Card: React.FC<CardProps> = ({
  children,
  title,
  onClick,
  hoverable = false,
  padding = 'md',
  style: customStyle,
}) => {
  const [isHovered, setIsHovered] = React.useState(false);

  const paddingStyles: Record<string, string> = {
    sm: theme.spacing.sm,
    md: theme.spacing.md,
    lg: theme.spacing.lg,
  };

  const baseStyle: React.CSSProperties = {
    background: theme.colors.background.dark,
    borderRadius: theme.borderRadius.lg,
    padding: paddingStyles[padding],
    boxShadow: theme.shadows.md,
    border: `1px solid ${theme.colors.background.darker}`,
    transition: theme.transitions.normal,
    cursor: onClick || hoverable ? 'pointer' : 'default',
    ...customStyle,
  };

  const hoverStyle: React.CSSProperties = {
    transform: 'translateY(-2px)',
    boxShadow: theme.shadows.lg,
    borderColor: theme.colors.primary,
  };

  const style: React.CSSProperties = {
    ...baseStyle,
    ...(isHovered && (onClick || hoverable) ? hoverStyle : {}),
  };

  return (
    <div
      style={style}
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      {title && (
        <h3
          style={{
            fontFamily: theme.fonts.heading,
            fontSize: theme.fontSizes.lg,
            color: theme.colors.text.primary,
            marginBottom: theme.spacing.sm,
            borderBottom: `1px solid ${theme.colors.background.darker}`,
            paddingBottom: theme.spacing.sm,
          }}
        >
          {title}
        </h3>
      )}
      {children}
    </div>
  );
};
