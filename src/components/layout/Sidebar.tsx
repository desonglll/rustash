import { Link, useLocation } from "react-router-dom";

interface SidebarProps {
  collapsed: boolean;
  onToggle: () => void;
}

const navItems = [
  { label: "Scenes", icon: "🎬", href: "/scenes" },
  { label: "Performers", icon: "👤", href: "/performers" },
  { label: "Studios", icon: "🏢", href: "/studios" },
  { label: "Tags", icon: "🏷️", href: "/tags" },
  { label: "Galleries", icon: "🖼️", href: "/galleries" },
  { label: "Images", icon: "📷", href: "/images" },
  { label: "Groups", icon: "🎞️", href: "/groups" },
  { label: "Settings", icon: "⚙️", href: "/settings" },
];

export default function Sidebar({ collapsed, onToggle }: SidebarProps) {
  const location = useLocation();

  return (
    <aside
      className={`bg-gray-900 text-gray-100 flex flex-col transition-all duration-200 ${
        collapsed ? "w-16" : "w-56"
      }`}
    >
      <div className="flex items-center justify-between h-14 px-3 border-b border-gray-700">
        {!collapsed && (
          <Link to="/" className="text-lg font-bold tracking-wide hover:text-white">
            Rustash
          </Link>
        )}
        <button
          onClick={onToggle}
          className="p-1.5 rounded hover:bg-gray-700 text-gray-400 hover:text-white"
          aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
        >
          {collapsed ? "▸" : "◂"}
        </button>
      </div>
      <nav className="flex-1 py-2 space-y-0.5">
        {navItems.map((item) => (
          <Link
            key={item.href}
            to={item.href}
            className={`flex items-center gap-3 px-3 py-2 text-sm rounded mx-1 ${
              location.pathname === item.href
                ? "bg-gray-700 text-white"
                : "hover:bg-gray-800 hover:text-white"
            }`}
          >
            <span className="text-base">{item.icon}</span>
            {!collapsed && <span>{item.label}</span>}
          </Link>
        ))}
      </nav>
    </aside>
  );
}
