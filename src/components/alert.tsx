import React, { createContext, useContext, useState, useCallback } from 'react';

// Alert button configuration
export interface AlertButton {
  label: string;
  action: () => void;
  variant?: 'primary' | 'secondary' | 'destructive';
  disabled?: boolean;
}

// Alert size options
export type AlertSize = 'small' | 'medium' | 'large' | 'full';

// Alert configuration
export interface AlertConfig {
  id: string;
  title?: string;
  message?: string;
  content?: React.ReactNode; // New: support for custom components
  buttons?: AlertButton[];
  type?: 'info' | 'success' | 'warning' | 'error';
  duration?: number; // Auto-dismiss duration in ms (0 = no auto-dismiss)
  closable?: boolean;
  size?: AlertSize; // New: size options
}

// Internal alert state
interface AlertState extends AlertConfig {
  isVisible: boolean;
  isExiting: boolean;
}

// Alert context
interface AlertContextType {
  alerts: AlertState[];
  showAlert: (config: Omit<AlertConfig, 'id'>) => string;
  hideAlert: (id: string) => void;
  hideAllAlerts: () => void;
}

const AlertContext = createContext<AlertContextType | null>(null);

// Custom hook to use alert context
export const useAlert = () => {
  const context = useContext(AlertContext);
  if (!context) {
    throw new Error('useAlert must be used within an AlertProvider');
  }
  return context;
};

// Alert Provider Component
export const AlertProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [alerts, setAlerts] = useState<AlertState[]>([]);

  const showAlert = useCallback((config: Omit<AlertConfig, 'id'>): string => {
    const id = Math.random().toString(36).substr(2, 9);
    const newAlert: AlertState = {
      ...config,
      id,
      isVisible: false,
      isExiting: false,
      closable: config.closable ?? true,
      type: config.type ?? 'info',
      size: config.size ?? 'medium',
    };

    setAlerts(prev => [...prev, newAlert]);

    // Trigger entrance animation - reduced from 10ms to 5ms for faster response
    setTimeout(() => {
      setAlerts(prev => 
        prev.map(alert => 
          alert.id === id ? { ...alert, isVisible: true } : alert
        )
      );
    }, 5);

    // Auto-dismiss if duration is set
    if (config.duration && config.duration > 0) {
      setTimeout(() => {
        hideAlert(id);
      }, config.duration);
    }

    return id;
  }, []);

  const hideAlert = useCallback((id: string) => {
    setAlerts(prev => 
      prev.map(alert => 
        alert.id === id ? { ...alert, isExiting: true } : alert
      )
    );

    // Remove from DOM after animation - reduced from 300ms to 200ms
    setTimeout(() => {
      setAlerts(prev => prev.filter(alert => alert.id !== id));
    }, 200);
  }, []);

  const hideAllAlerts = useCallback(() => {
    setAlerts(prev => 
      prev.map(alert => ({ ...alert, isExiting: true }))
    );

    setTimeout(() => {
      setAlerts([]);
    }, 200);
  }, []);

  return (
    <AlertContext.Provider value={{ alerts, showAlert, hideAlert, hideAllAlerts }}>
      {children}
      <AlertContainer />
    </AlertContext.Provider>
  );
};

// Alert Container Component
const AlertContainer: React.FC = () => {
  const { alerts, hideAlert } = useAlert();
  const hasAlerts = alerts.length > 0;

  return (
    <>
      {/* Backdrop */}
      <div
        className={`fixed inset-0 z-50 transition-all duration-200 ease-out ${
          hasAlerts 
            ? 'backdrop-blur-sm bg-black/20 opacity-100' 
            : 'opacity-0 pointer-events-none backdrop-blur-none'
        }`}
        onClick={() => {
          // Close the topmost closable alert when clicking backdrop
          const topAlert = alerts[alerts.length - 1];
          if (topAlert?.closable) {
            hideAlert(topAlert.id);
          }
        }}
      />

      {/* Alerts Container */}
      <div className="fixed inset-0 z-50 flex items-center justify-center p-4 pointer-events-none text-center dir-fa">
        <div className="relative flex flex-col items-center">
          {alerts.map((alert, index) => (
            <AlertComponent
              key={alert.id}
              alert={alert}
              index={index}
              totalAlerts={alerts.length}
              onClose={() => hideAlert(alert.id)}
            />
          ))}
        </div>
      </div>
    </>
  );
};

// Individual Alert Component
interface AlertComponentProps {
  alert: AlertState;
  index: number;
  totalAlerts: number;
  onClose: () => void;
}

const AlertComponent: React.FC<AlertComponentProps> = ({ alert, index, totalAlerts, onClose }) => {
  const isTopAlert = index === totalAlerts - 1;
  const translateY = (totalAlerts - 1 - index) * -8; // Stack effect
  const scale = 1 - (totalAlerts - 1 - index) * 0.02; // Subtle scale effect
  const opacity = isTopAlert ? 1 : 0.8 - (totalAlerts - 1 - index) * 0.1;

  const getTypeStyles = (type: AlertConfig['type']) => {
    switch (type) {
      case 'success':
        return 'border-l-green-500 bg-green-500/10';
      case 'warning':
        return 'border-l-yellow-500 bg-yellow-500/10';
      case 'error':
        return 'border-l-red-500 bg-red-500/10';
      default:
        return 'border-l-blue-500 bg-blue-500/10';
    }
  };

  const getButtonStyles = (variant: AlertButton['variant']) => {
    switch (variant) {
      case 'primary':
        return 'bg-blue-600 hover:bg-blue-700 text-white';
      case 'destructive':
        return 'bg-red-600 hover:bg-red-700 text-white';
      default:
        return 'bg-gray-600 hover:bg-gray-700 text-white';
    }
  };

  // Get size-based styles
  const getSizeStyles = (size: AlertSize = 'medium') => {
    switch (size) {
      case 'small':
        return {
          container: 'min-w-64 max-w-sm',
          padding: 'p-3',
          title: 'text-base',
          text: 'text-sm',
          button: 'px-3 py-1.5 text-sm'
        };
      case 'large':
        return {
          container: 'min-w-96 max-w-2xl',
          padding: 'p-6',
          title: 'text-2xl',
          text: 'text-base',
          button: 'px-6 py-3 text-base'
        };
      case 'full':
        return {
          container: 'w-[90vw] max-w-4xl',
          padding: 'p-8',
          title: 'text-3xl',
          text: 'text-lg',
          button: 'px-8 py-4 text-lg'
        };
      default: // medium
        return {
          container: 'min-w-80 max-w-md',
          padding: 'p-4',
          title: 'text-lg',
          text: 'text-base',
          button: 'px-4 py-2 text-base'
        };
    }
  };

  const sizeStyles = getSizeStyles(alert.size);

  return (
    <div
      className={`pointer-events-auto transition-all duration-200 ${
        alert.isVisible && !alert.isExiting
          ? 'translate-y-0 opacity-100 scale-100'
          : alert.isExiting
          ? '-translate-y-2 opacity-0 scale-95'
          : 'translate-y-2 opacity-0 scale-95'
      } ${index > 0 ? 'absolute' : ''}`}
      style={{
        transform: index > 0 ? `translateY(${translateY}px) scale(${scale})` : undefined,
        opacity: index > 0 ? opacity : undefined,
        zIndex: 50 + index,
        top: index > 0 ? '0' : undefined,
        left: index > 0 ? '50%' : undefined,
        marginLeft: index > 0 ? '-50%' : undefined,
      }}
    >
      <div className={`
        ${sizeStyles.container} mx-auto rounded-lg shadow-2xl border-l-4
        transition-all duration-150 ease-out
        ${getTypeStyles(alert.type)}
      `}
      style={{ 
        backgroundColor: '#444C56',
        border: '1px solid #30363D',
        borderLeftColor: alert.type === 'success' ? '#22c55e' : 
                       alert.type === 'warning' ? '#eab308' : 
                       alert.type === 'error' ? '#ef4444' : '#3b82f6'
      }}>
        {/* Header */}
        <div className={`flex items-start justify-between ${sizeStyles.padding} ${alert.buttons && alert.buttons.length > 0 ? 'pb-2' : ''}`}>
          <div className="flex-1">
            {alert.title && (
              <h3 className={`${sizeStyles.title} font-semibold text-white mb-1`}>
                {alert.title}
              </h3>
            )}
            
            {/* Support for both message string and custom content */}
            {alert.content ? (
              <div className="text-gray-200">
                {alert.content}
              </div>
            ) : alert.message && (
              <p className={`${sizeStyles.text} text-gray-200 leading-relaxed`}>
                {alert.message}
              </p>
            )}
          </div>
          
          {alert.closable && (
            <button
              onClick={onClose}
              className="ml-4 text-gray-400 hover:text-white transition-colors duration-150 p-1 rounded-md hover:bg-white/10"
              aria-label="Close alert"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          )}
        </div>

        {/* Buttons */}
        {alert.buttons && alert.buttons.length > 0 && (
          <div className={`px-${alert.size === 'small' ? '3' : alert.size === 'large' || alert.size === 'full' ? '6' : '4'} pb-${alert.size === 'small' ? '3' : alert.size === 'large' || alert.size === 'full' ? '6' : '4'} pt-2`}>
            <div className="flex gap-2 justify-end flex-wrap">
              {alert.buttons.map((button, buttonIndex) => (
                <button
                  key={buttonIndex}
                  onClick={() => {
                    button.action();
                    onClose();
                  }}
                  disabled={button.disabled}
                  className={`
                    ${sizeStyles.button} rounded-md font-medium transition-all duration-150
                    disabled:opacity-50 disabled:cursor-not-allowed
                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-800
                    ${getButtonStyles(button.variant)}
                  `}
                >
                  {button.label}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

// Convenience functions for common alert types
export const useAlertHelpers = () => {
  const { showAlert } = useAlert();

  return {
    showInfo: (message: string, options?: Partial<AlertConfig>) =>
      showAlert({ ...options, message, type: 'info' }),
    
    showSuccess: (message: string, options?: Partial<AlertConfig>) =>
      showAlert({ ...options, message, type: 'success' }),
    
    showWarning: (message: string, options?: Partial<AlertConfig>) =>
      showAlert({ ...options, message, type: 'warning' }),
    
    showError: (message: string, options?: Partial<AlertConfig>) =>
      showAlert({ ...options, message, type: 'error' }),
    
    showConfirm: (message: string, onConfirm: () => void, options?: Partial<AlertConfig>) =>
      showAlert({
        ...options,
        message,
        type: 'warning',
        buttons: [
          { label: 'Cancel', action: () => {}, variant: 'secondary' },
          { label: 'Confirm', action: onConfirm, variant: 'primary' }
        ]
      }),

    // New: convenience function for custom content
    showCustom: (content: React.ReactNode, options?: Partial<AlertConfig>) =>
      showAlert({ ...options, content }),
  };
};

// Export default component for backward compatibility
export default function Alert() {
  return <AlertProvider><div>Alert System Ready</div></AlertProvider>;
}