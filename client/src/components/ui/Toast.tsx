import React, { useEffect } from 'react';
import { theme } from '../../theme';

export interface ToastProps {
  message: string;
  type?: 'success' | 'error' | 'info';
  duration?: number;
  onClose: () => void;
  isVisible: boolean;
}

export const Toast: React.FC<ToastProps> = ({
  message,
  type = 'info',
  duration = 3000,
  onClose,
  isVisible,
}) => {
  useEffect(() => {
    if (isVisible && duration > 0) {
      const timer = setTimeout(() => {
        onClose();
      }, duration);
      return () => clearTimeout(timer);
    }
  }, [isVisible, duration, onClose]);

  if (!isVisible) return null;

  const typeStyles: Record<string, React.CSSProperties> = {
    success: {
      background: theme.colors.success,
      color: theme.colors.text.primary,
    },
    error: {
      background: theme.colors.danger,
      color: theme.colors.text.primary,
    },
    info: {
      background: theme.colors.secondary,
      color: theme.colors.text.primary,
    },
  };

  const toastStyle: React.CSSProperties = {
    position: 'fixed',
    bottom: theme.spacing.xl,
    right: theme.spacing.xl,
    padding: `${theme.spacing.md} ${theme.spacing.lg}`,
    borderRadius: theme.borderRadius.md,
    boxShadow: theme.shadows.lg,
    fontFamily: theme.fonts.body,
    fontSize: theme.fontSizes.md,
    fontWeight: '500',
    zIndex: theme.zIndex.notification,
    animation: 'slideIn 0.3s ease-out',
    minWidth: '250px',
    maxWidth: '400px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: theme.spacing.md,
    ...typeStyles[type],
  };

  const closeButtonStyle: React.CSSProperties = {
    background: 'transparent',
    border: 'none',
    color: 'inherit',
    fontSize: theme.fontSizes.lg,
    cursor: 'pointer',
    padding: 0,
    lineHeight: 1,
    opacity: 0.8,
    transition: theme.transitions.fast,
  };

  return (
    <>
      <style>
        {`
          @keyframes slideIn {
            from {
              transform: translateX(400px);
              opacity: 0;
            }
            to {
              transform: translateX(0);
              opacity: 1;
            }
          }
        `}
      </style>
      <div style={toastStyle}>
        <span>{message}</span>
        <button
          style={closeButtonStyle}
          onClick={onClose}
          onMouseEnter={(e) => {
            e.currentTarget.style.opacity = '1';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.opacity = '0.8';
          }}
        >
          ×
        </button>
      </div>
    </>
  );
};

// Hook for managing toasts
export function useToast() {
  const [toast, setToast] = React.useState<{
    message: string;
    type: 'success' | 'error' | 'info';
    isVisible: boolean;
  }>({
    message: '',
    type: 'info',
    isVisible: false,
  });

  const showToast = React.useCallback((message: string, type: 'success' | 'error' | 'info' = 'info') => {
    setToast({ message, type, isVisible: true });
  }, []);

  const hideToast = React.useCallback(() => {
    setToast((prev) => ({ ...prev, isVisible: false }));
  }, []);

  return {
    toast,
    showToast,
    hideToast,
  };
}
