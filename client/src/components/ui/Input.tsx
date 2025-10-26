import React from 'react';
import { theme } from '../../theme';

export interface InputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  type?: 'text' | 'password' | 'email' | 'number';
  disabled?: boolean;
  fullWidth?: boolean;
  label?: string;
  error?: string;
}

export const Input: React.FC<InputProps> = ({
  value,
  onChange,
  placeholder,
  type = 'text',
  disabled = false,
  fullWidth = false,
  label,
  error,
}) => {
  const [isFocused, setIsFocused] = React.useState(false);

  const containerStyle: React.CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    gap: theme.spacing.xs,
    width: fullWidth ? '100%' : 'auto',
  };

  const labelStyle: React.CSSProperties = {
    fontFamily: theme.fonts.body,
    fontSize: theme.fontSizes.sm,
    color: theme.colors.text.secondary,
    fontWeight: '500',
  };

  const inputStyle: React.CSSProperties = {
    fontFamily: theme.fonts.body,
    fontSize: theme.fontSizes.md,
    padding: `${theme.spacing.sm} ${theme.spacing.md}`,
    background: theme.colors.background.darker,
    border: `2px solid ${error ? theme.colors.danger : isFocused ? theme.colors.primary : theme.colors.background.darker}`,
    borderRadius: theme.borderRadius.md,
    color: theme.colors.text.primary,
    outline: 'none',
    transition: theme.transitions.normal,
    width: fullWidth ? '100%' : '300px',
    opacity: disabled ? 0.5 : 1,
    cursor: disabled ? 'not-allowed' : 'text',
  };

  const errorStyle: React.CSSProperties = {
    fontFamily: theme.fonts.body,
    fontSize: theme.fontSizes.sm,
    color: theme.colors.danger,
  };

  return (
    <div style={containerStyle}>
      {label && <label style={labelStyle}>{label}</label>}
      <input
        type={type}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
        style={inputStyle}
        onFocus={() => setIsFocused(true)}
        onBlur={() => setIsFocused(false)}
      />
      {error && <span style={errorStyle}>{error}</span>}
    </div>
  );
};
