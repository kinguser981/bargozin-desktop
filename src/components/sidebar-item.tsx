import { Link, useLocation } from "react-router";

export default function SidebarItem(props: {
  icon: React.ReactNode;
  title: string;
  href: string;
}) {
  const location = useLocation();
  const isActive = location.pathname === props.href;
  
  return (
    <Link
      to={props.href}
      className={`flex items-center justify-end gap-2 text-right mb-[15px] text-sm rounded-lg px-2 py-1 ${
        isActive
          ? "bg-gradient-to-br from-[#1C4C91] to-[#2F81F7]"
          : "hover:bg-[#122239]"
      }`}
    >
      <span>{props.title}</span>
      <span>{props.icon}</span>
    </Link>
  );
}
