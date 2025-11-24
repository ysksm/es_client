import { HTMLAttributes, ReactNode } from 'react';

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  title?: string;
  children: ReactNode;
}

export const Card = ({ title, children, className = '', ...props }: CardProps) => {
  return (
    <div className={`card ${className}`} {...props}>
      {title && (
        <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
          {title}
        </h3>
      )}
      {children}
    </div>
  );
};
