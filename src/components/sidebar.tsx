import SidebarItem from "./sidebar-item";
import Docker from "./svg/docker";
import Download from "./svg/download";
import Info from "./svg/info";
import Web from "./svg/web";

export default function Sidebar() {
  return (
    <div className="bg-[#161B22] w-[245px] text-right px-2 py-4 rounded-3xl h-full flex flex-col justify-between min-h-0">
      <div className="px-4">
        <h3 className="text-white text-lg font-light text-right mt-2">
          سرویس ها
        </h3>
        <div className="mt-4">
          <SidebarItem icon={<Web />} title="تست دامنه" href="/" />
          <SidebarItem
            icon={<Download />}
            title="دانلود بهینه فایل"
            href="/download"
          />
          <SidebarItem icon={<Docker />} title="تست داکر" href="/docker" />
        </div>
      </div>
      <div className="border-t pt-4 border-[#444C56]">
        <SidebarItem icon={<Info />} title="درباره ما" href="/about" />
      </div>
    </div>
  );
}
