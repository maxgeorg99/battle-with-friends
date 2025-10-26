import React from 'react';
import { theme } from '../../theme';

export interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  variant?: 'primary' | 'secondary' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  fullWidth?: boolean;
  type?: 'button' | 'submit' | 'reset';
}

export const Button: React.FC<ButtonProps> = ({
  children,
  onClick,
  variant = 'primary',
  size = 'md',
  disabled = false,
  fullWidth = false,
  type = 'button',
}) => {
  const baseStyle: React.CSSProperties = {
    fontFamily: theme.fonts.body,
    border: 'none',
    borderRadius: theme.borderRadius.md,
    cursor: disabled ? 'not-allowed' : 'pointer',
    transition: theme.transitions.normal,
    fontWeight: '600',
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    width: fullWidth ? '100%' : 'auto',
    opacity: disabled ? 0.5 : 1,
    boxShadow: theme.shadows.sm,
  };

  const variantStyles: Record<string, React.CSSProperties> = {
    primary: {
      background: theme.colors.primary,
      color: theme.colors.background.darkest,
    },
    secondary: {
      background: theme.colors.secondary,
      color: theme.colors.text.primary,
    },
    danger: {
      background: theme.colors.danger,
      color: theme.colors.text.primary,
    },
  };

  const sizeStyles: Record<string, React.CSSProperties> = {
    sm: {
      padding: `${theme.spacing.xs} ${theme.spacing.md}`,
      fontSize: theme.fontSizes.sm,
    },
    md: {
      padding: `${theme.spacing.sm} ${theme.spacing.lg}`,
      fontSize: theme.fontSizes.md,
    },
    lg: {
      padding: `${theme.spacing.md} ${theme.spacing.xl}`,
      fontSize: theme.fontSizes.lg,
    },
  };

  const [isHovered, setIsHovered] = React.useState(false);
  const [isActive, setIsActive] = React.useState(false);

  const style: React.CSSProperties = {
    ...baseStyle,
    ...variantStyles[variant],
    ...sizeStyles[size],
    ...(isHovered && !disabled && { filter: 'brightness(1.1)', transform: 'translateY(-1px)' }),
    ...(isActive && !disabled && { transform: 'translateY(0px)' }),
  };

  return (
    <button
      type={type}
      onClick={disabled ? undefined : onClick}
      style={style}
      disabled={disabled}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => {
        setIsHovered(false);
        setIsActive(false);
      }}
      onMouseDown={() => setIsActive(true)}
      onMouseUp={() => setIsActive(false)}
    >
      {children}
    </button>
  );
};
