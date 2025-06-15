import Spinner from './spinner';

interface LoadingProps {
  text?: string;
  fullScreen?: boolean;
  className?: string;
  onCancel?: () => void;
  showCancel?: boolean;
  cancelText?: string;
}

export default function Loading({ 
  text = 'در حال بررسی...',
  fullScreen = true,
  className = '',
  onCancel,
  showCancel = false,
  cancelText = 'لغو'
}: LoadingProps) {
  const containerClasses = fullScreen 
    ? 'fixed inset-0 bg-opacity-60 backdrop-blur-sm flex items-center justify-center z-50'
    : 'flex items-center justify-center py-8';

  return (
    <div 
      className={`${containerClasses} ${className}`}
      role="status" 
      aria-live="polite"
    >
      <div className="rounded-2xl p-8 mx-4 max-w-sm w-full">
        <div className="flex flex-col items-center space-y-6">
          {/* Spinner */}
          <Spinner />
          
          {/* Text */}
          {text && (
            <div className="text-center">
              <p className="text-lg font-medium text-gray-800 dark:text-gray-200 dir-fa">
                {text}
              </p>
            </div>
          )}
          
          {/* Cancel Button */}
          {showCancel && onCancel && (
            <button
              onClick={onCancel}
              className="
                px-6 py-3
                font-medium 
                rounded-lg 
                dir-fa
                cursor-pointer
              "
              type="button"
            >
              {cancelText}
            </button>
          )}
        </div>
      </div>
    </div>
  );
}