import React, { useEffect } from 'react';
import { theme } from '../../theme';

export interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  children: React.ReactNode;
  maxWidth?: string;
  showCloseButton?: boolean;
}

export const Modal: React.FC<ModalProps> = ({
  isOpen,
  onClose,
  title,
  children,
  maxWidth = '600px',
  showCloseButton = true,
}) => {
  // Close on ESC key
  useEffect(() => {
    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };
    window.addEventListener('keydown', handleEsc);
    return () => window.removeEventListener('keydown', handleEsc);
  }, [isOpen, onClose]);

  // Prevent body scroll when modal is open
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
    return () => {
      document.body.style.overflow = '';
    };
  }, [isOpen]);

  if (!isOpen) return null;

  const overlayStyle: React.CSSProperties = {
    position: 'fixed',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    background: theme.colors.overlay,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    zIndex: theme.zIndex.modal,
    padding: theme.spacing.md,
  };

  const modalStyle: React.CSSProperties = {
    background: theme.colors.background.dark,
    borderRadius: theme.borderRadius.lg,
    boxShadow: theme.shadows.lg,
    maxWidth,
    width: '100%',
    maxHeight: '90vh',
    overflow: 'auto',
    position: 'relative',
    border: `2px solid ${theme.colors.primary}`,
  };

  const headerStyle: React.CSSProperties = {
    padding: theme.spacing.lg,
    borderBottom: `1px solid ${theme.colors.background.darker}`,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
  };

  const titleStyle: React.CSSProperties = {
    fontFamily: theme.fonts.heading,
    fontSize: theme.fontSizes.xl,
    color: theme.colors.text.primary,
    margin: 0,
  };

  const closeButtonStyle: React.CSSProperties = {
    background: 'transparent',
    border: 'none',
    color: theme.colors.text.secondary,
    fontSize: theme.fontSizes.xl,
    cursor: 'pointer',
    padding: theme.spacing.sm,
    lineHeight: 1,
    transition: theme.transitions.fast,
  };

  const contentStyle: React.CSSProperties = {
    padding: theme.spacing.lg,
  };

  return (
    <div style={overlayStyle} onClick={onClose}>
      <div style={modalStyle} onClick={(e) => e.stopPropagation()}>
        {(title || showCloseButton) && (
          <div style={headerStyle}>
            {title && <h2 style={titleStyle}>{title}</h2>}
            {showCloseButton && (
              <button
                style={closeButtonStyle}
                onClick={onClose}
                onMouseEnter={(e) => {
                  e.currentTarget.style.color = theme.colors.text.primary;
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.color = theme.colors.text.secondary;
                }}
              >
                ×
              </button>
            )}
          </div>
        )}
        <div style={contentStyle}>{children}</div>
      </div>
    </div>
  );
};
