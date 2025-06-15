import { Link, useLocation } from "react-router";

export default function SidebarItem(props: {
  icon: React.ReactNode;
  title: string;
  href: string;
}) {
  const location = useLocation();
  return (
    <Link
      to={props.href}
      className={`flex items-center justify-end gap-2 text-right mb-[15px] text-lg hover:bg-gradient-to-bl hover:from-[#1C4C91] hover:to-[#2F81F7] transition duration-300 rounded-lg px-3 py-2 ${
        location.pathname === props.href
          ? "bg-gradient-to-br from-[#1C4C91] to-[#2F81F7]"
          : ""
      }`}
    >
      <div>{props.title}</div>
      <div>{props.icon}</div>
    </Link>
  );
}
