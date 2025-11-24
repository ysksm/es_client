import { NavLink } from 'react-router-dom';
import {
  HomeIcon,
  ServerIcon,
  CircleStackIcon,
  ArrowDownTrayIcon,
  TableCellsIcon,
} from '@heroicons/react/24/outline';

const navItems = [
  { name: 'Dashboard', path: '/', icon: HomeIcon },
  { name: 'Connections', path: '/connections', icon: ServerIcon },
  { name: 'Indices', path: '/indices', icon: CircleStackIcon },
  { name: 'Extract', path: '/extract', icon: ArrowDownTrayIcon },
  { name: 'Database', path: '/database', icon: TableCellsIcon },
];

export const Sidebar = () => {
  return (
    <aside className="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700">
      <nav className="p-4 space-y-1">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) =>
              `flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                isActive
                  ? 'bg-primary-100 dark:bg-primary-900 text-primary-900 dark:text-primary-100'
                  : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'
              }`
            }
          >
            <item.icon className="w-5 h-5" />
            <span className="font-medium">{item.name}</span>
          </NavLink>
        ))}
      </nav>
    </aside>
  );
};
