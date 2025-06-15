import { useAlertHelpers } from "../components/alert";
import DoubleChevronDown from "../components/svg/double-chevron-down";
import Question from "../components/svg/question";
import Search from "../components/svg/search";
import TestResultItem from "../components/test-result-item";
import { useRef, useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface DnsTestResult {
  dns_server: string;
  status: boolean;
  response_time?: number;
  error_message?: string;
}

export default function DomainTest() {
  // return <Loading onCancel={() => {}} showCancel={true} cancelText="انصراف" />
  const { showInfo, showError } = useAlertHelpers();
  const leftColumnRef = useRef<HTMLDivElement>(null);
  const rightColumnRef = useRef<HTMLDivElement>(null);
  
  const [domain, setDomain] = useState("");
  const [usableResults, setUsableResults] = useState<DnsTestResult[]>([]);
  const [unusableResults, setUnusableResults] = useState<DnsTestResult[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isCompleted, setIsCompleted] = useState(false);

  const scrollToBottom = (ref: React.RefObject<HTMLDivElement>) => {
    if (ref.current) {
      ref.current.scrollTo({
        top: ref.current.scrollHeight,
        behavior: "smooth",
      });
    }
  };

  useEffect(() => {
    // Listen for individual DNS test results
    const unlisten = listen<DnsTestResult>("dns-test-result", (event) => {
      const result = event.payload;
      
      if (result.status) {
        setUsableResults(prev => [...prev, result]);
        // Auto-scroll right column when new usable result arrives
        setTimeout(() => scrollToBottom(rightColumnRef), 100);
      } else {
        setUnusableResults(prev => [...prev, result]);
        // Auto-scroll left column when new unusable result arrives
        setTimeout(() => scrollToBottom(leftColumnRef), 100);
      }
    });

    // Listen for completion event
    const unlistenComplete = listen("dns-test-complete", () => {
      setIsLoading(false);
      setIsCompleted(true);
    });

    // Cleanup listeners on component unmount
    return () => {
      unlisten.then(fn => fn());
      unlistenComplete.then(fn => fn());
    };
  }, []);

  const handleDnsTest = async () => {
    if (!domain.trim()) {
      showError("لطفاً یک دامنه وارد کنید");
      return;
    }

    setIsLoading(true);
    setIsCompleted(false);
    setUsableResults([]);
    setUnusableResults([]);

    try {
      await invoke("test_dns_servers", {
        domain: domain.trim(),
      });
    } catch (error) {
      console.error("DNS test error:", error);
      showError("خطا در انجام تست DNS: " + error);
      setIsLoading(false);
    }
  };

  const totalResults = usableResults.length + unusableResults.length;
  const totalExpected = 19; // Total number of DNS servers

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
            value={domain}
            onChange={(e) => setDomain(e.target.value)}
            onKeyPress={(e) => e.key === "Enter" && handleDnsTest()}
            className="bg-[#30363D] border border-[#6B7280] rounded-md p-4 text-sm w-full text-right dir-fa focus:outline-none focus:border-[#8B9DC3]"
            placeholder="مثلا spotify.com"
            disabled={isLoading}
          />
          <button 
            onClick={handleDnsTest}
            disabled={isLoading}
            className="group dir-fa absolute left-2 top-[7px] p-2 px-5 transition rounded bg-[#96989A] text-[#848484] flex items-center gap-2 cursor-pointer hover:bg-[#38727C] hover:text-white disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Search />
            {isLoading ? "در حال بررسی..." : "بررسی DNS ها"}
          </button>
        </div>
      </div>
      
      <div>
        <div className="flex justify-between items-center mb-4">
          <p className="text-center flex-1">نتایج تست</p>
          {isLoading && (
            <div className="text-sm text-gray-400">
              {totalResults}/{totalExpected} تست شده
            </div>
          )}
        </div>

        {isLoading && totalResults === 0 && (
          <div className="text-center py-8">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-[#8B9DC3]"></div>
            <p className="mt-2 text-gray-400">در حال شروع تست DNS سرورها...</p>
          </div>
        )}
        
        {(totalResults > 0 || isCompleted) && (
          <div className="grid grid-cols-2 gap-4 my-4">
            {/* Left Column - Unusable DNS servers */}
            <div className="relative">
              <div className="mb-2 text-center">
                <span className="text-red-400 text-sm font-medium">
                  مسدود شده ({unusableResults.length})
                </span>
              </div>
              <div
                ref={leftColumnRef}
                className="max-h-[450px] overflow-auto scrollbar-thin scrollbar-thumb-gray-600 scrollbar-track-gray-800"
              >
                {unusableResults.map((result, index) => (
                  <TestResultItem 
                    key={`unusable-${index}`}
                    dns={result.dns_server} 
                    status={result.status}
                    responseTime={result.response_time}
                    errorMessage={result.error_message}
                  />
                ))}
                {unusableResults.length === 0 && isCompleted && (
                  <div className="text-center py-8 text-gray-400">
                    <p>هیچ سرور DNS مسدودی یافت نشد!</p>
                  </div>
                )}
              </div>

              {unusableResults.length > 5 && (
                <>
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
                </>
              )}
            </div>

            {/* Right Column - Usable DNS servers */}
            <div className="relative">
              <div className="mb-2 text-center">
                <span className="text-green-400 text-sm font-medium">
                  قابل استفاده ({usableResults.length})
                </span>
              </div>
              <div
                ref={rightColumnRef}
                className="max-h-[450px] overflow-auto scrollbar-thin scrollbar-thumb-gray-600 scrollbar-track-gray-800"
              >
                {usableResults.map((result, index) => (
                  <TestResultItem 
                    key={`usable-${index}`}
                    dns={result.dns_server} 
                    status={result.status}
                    responseTime={result.response_time}
                    errorMessage={result.error_message}
                  />
                ))}
                {usableResults.length === 0 && isCompleted && (
                  <div className="text-center py-8 text-gray-400">
                    <p>متأسفانه هیچ سرور DNS قابل استفاده‌ای یافت نشد</p>
                  </div>
                )}
              </div>

              {usableResults.length > 5 && (
                <>
                  {/* Black Gradient Overlay */}
                  <div className="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-[#0D1117] to-transparent pointer-events-none"></div>

                  {/* More Items Button */}
                  <div className="absolute bottom-2 left-1/2 transform -translate-x-1/2">
                    <button
                      onClick={() => scrollToBottom(rightColumnRef)}
                      className="text-gray-300 hover:text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors duration-200 shadow-lg dir-fa flex items-center gap-2"
                    >
                      <DoubleChevronDown />
                      موارد بیشتر
                    </button>
                  </div>
                </>
              )}
            </div>
          </div>
        )}

        {isCompleted && (
          <div className="mt-4 text-center">
            <div className="inline-flex items-center gap-4 bg-[#30363D] rounded-lg px-6 py-3">
              <div className="text-green-400">
                <span className="font-medium">{usableResults.length}</span> قابل استفاده
              </div>
              <div className="text-gray-400">|</div>
              <div className="text-red-400">
                <span className="font-medium">{unusableResults.length}</span> مسدود شده
              </div>
              <div className="text-gray-400">|</div>
              <div className="text-gray-300">
                مجموع: <span className="font-medium">{totalResults}</span>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
