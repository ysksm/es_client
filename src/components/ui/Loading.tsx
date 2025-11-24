interface LoadingProps {
  size?: 'sm' | 'md' | 'lg';
  text?: string;
}

export const Loading = ({ size = 'md', text }: LoadingProps) => {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-8 h-8',
    lg: 'w-12 h-12',
  };

  return (
    <div className="flex flex-col items-center justify-center gap-3 p-8">
      <div
        className={`${sizeClasses[size]} border-4 border-primary-200 dark:border-primary-900 border-t-primary-600 dark:border-t-primary-400 rounded-full animate-spin`}
      />
      {text && (
        <p className="text-gray-600 dark:text-gray-400">{text}</p>
      )}
    </div>
  );
};
