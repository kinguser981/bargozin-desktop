import Sidebar from "./sidebar";

export default function Layout(props: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-[#0D1117]">
      <header className="h-[128px] gap-4 text-white bg-gradient-to-b from-[#30363D80] to-[var(--color-bg)] flex items-center justify-end px-10">
        <div className="text-right">
          <h1 className="text-2xl font-bold">برگزین</h1>
          <p className="text -sm">انتخاب بهترین گزینه</p>
        </div>
        <div className="bg-gray-500 h-[56px] w-[56px] rounded-md"></div>
      </header>
      <section className="mr-10 mx-auto px-4 py-8 text-white flex max-h-[615px]">
        <main className="flex-1">
          {props.children}
        </main>
        <Sidebar />
      </section>
    </div>
  );
}
