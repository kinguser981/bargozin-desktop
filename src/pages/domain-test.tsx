import { useAlertHelpers } from "../components/alert";
import DoubleChevronDown from "../components/svg/double-chevron-down";
import Question from "../components/svg/question";
import Search from "../components/svg/search";
import TestResultItem from "../components/test-result-item";
import { useRef } from "react";

export default function DomainTest() {
  // return <Loading onCancel={() => {}} showCancel={true} cancelText="انصراف" />
  const { showInfo } = useAlertHelpers();
  const leftColumnRef = useRef<HTMLDivElement>(null);
  const rightColumnRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = (ref: React.RefObject<HTMLDivElement>) => {
    if (ref.current) {
      ref.current.scrollTo({
        top: ref.current.scrollHeight,
        behavior: "smooth",
      });
    }
  };

  return (
    <div className="text-right px-10">
      <div>
        <p className="mb-4 flex justify-end items-center gap-2">
          <button
            className="cursor-pointer"
            onClick={() =>
              showInfo(
                "دامنه موردنظر خود را وارد کنید تا بررسی کنیم کدام سرورهای DNS می‌توانند آن را با موفقیت باز کنند."
              )
            }
          >
            <Question className="w-5 h-5" />
          </button>
          دامنه مورد نظر
        </p>
        <div className="mb-4 relative">
          <input
            type="text"
            className="bg-[#30363D] border border-[#6B7280] rounded-md p-4 text-sm w-full text-right dir-fa focus:outline-none focus:border-[#8B9DC3]"
            placeholder="مثلا spotify.com"
          />
          <button className="group dir-fa absolute left-2 top-[7px] p-2 px-5 transition rounded bg-[#96989A] text-[#848484] flex items-center gap-2 cursor-pointer hover:bg-[#38727C] hover:text-white">
            <Search />
            بررسی DNS ها
          </button>
        </div>
      </div>
      <div>
        <p className="text-center">نتایج تست</p>
        <div className="grid grid-cols-2 gap-4 my-4">
          {/* Left Column */}
          <div className="relative">
            <div
              ref={leftColumnRef}
              className="max-h-[450px] overflow-auto scrollbar-thin scrollbar-thumb-gray-600 scrollbar-track-gray-800"
            >
              <TestResultItem dns="1.1.1.1" status={true} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={true} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="8.8.8.8" status={true} />
            </div>

            {/* Black Gradient Overlay */}
            <div className="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-[#0D1117] to-transparent pointer-events-none"></div>

            {/* More Items Button */}
            <div className="absolute bottom-2 left-1/2 transform -translate-x-1/2">
              <button
                onClick={() => scrollToBottom(leftColumnRef)}
                className="text-gray-300 hover:text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors duration-200 shadow-lg dir-fa flex items-center gap-2"
              >
                <DoubleChevronDown />
                موارد بیشتر
              </button>
            </div>
          </div>

          {/* Right Column */}
          <div className="relative">
            <div
              ref={rightColumnRef}
              className="max-h-[450px] overflow-auto scrollbar-thin scrollbar-thumb-gray-600 scrollbar-track-gray-800"
            >
              <TestResultItem dns="1.1.1.1" status={true} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="8.8.8.8" status={true} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={false} />
              <TestResultItem dns="1.1.1.1" status={true} />
              <TestResultItem dns="1.1.1.1" status={false} />
            </div>

            {/* Black Gradient Overlay */}
            <div className="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-[#0D1117] to-transparent pointer-events-none"></div>

            {/* More Items Button */}
            <div className="absolute bottom-2 left-1/2 transform -translate-x-1/2">
              <button
                onClick={() => scrollToBottom(rightColumnRef)}
                className="text-gray-300 hover:text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors duration-200 shadow-lg dir-fa flex items-center"
              >
                <DoubleChevronDown />
                موارد بیشتر
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
