import Clipboard from "./svg/clipboard";
import { useState } from "react";

export default function TestResultItem(props: {
  dns: string;
  status: boolean;
  responseTime?: number;
  errorMessage?: string;
}) {
  const [isCopied, setIsCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(props.dns);
      setIsCopied(true);
      // Reset after 2 seconds
      setTimeout(() => setIsCopied(false), 2000);
    } catch (error) {
      console.error("Failed to copy:", error);
    }
  };

  const formatResponseTime = (time?: number) => {
    if (time === undefined) return "";
    return time < 1000 ? `${time}ms` : `${(time / 1000).toFixed(1)}s`;
  };

  const shortenErrorMessage = (message?: string) => {
    if (!message) return "";

    // Common DNS error patterns and their shortened versions
    const errorMappings = [
      {
        pattern: /DNS lookup failed: request timed out/i,
        short: "درخواست منقضی شد",
      },
      {
        pattern: /DNS lookup failed: connection refused/i,
        short: "اتصال رد شد",
      },
      { pattern: /DNS lookup failed: no response/i, short: "پاسخی دریافت نشد" },
      {
        pattern: /DNS lookup failed: network unreachable/i,
        short: "شبکه در دسترس نیست",
      },
      { pattern: /DNS lookup failed: server failure/i, short: "خطای سرور" },
      { pattern: /Invalid DNS server IP/i, short: "IP سرور نامعتبر" },
      { pattern: /No IP addresses found/i, short: "آدرس IP یافت نشد" },
      { pattern: /DNS lookup failed: no record/i, short: "رکوردی یافت نشد" },
    ];

    for (const mapping of errorMappings) {
      if (mapping.pattern.test(message)) {
        return mapping.short;
      }
    }

    // If no pattern matches, return truncated version
    return message.length > 35 ? message.substring(0, 32) + "..." : message;
  };

  return (
    <div
      className={`${
        props.status ? "bg-[#142A20]" : "bg-[#301B1F]"
      } h-[70px] rounded-lg mb-2 flex justify-between items-center px-1 dir-en`}
    >
      <div className="flex flex-col">
        <p className="flex items-center mb-1">
          <button
            className={`ml-2 p-1 rounded transition-all duration-200 hover:bg-white/10 ${
              isCopied 
                ? "text-green-400 scale-110" 
                : "text-gray-400 hover:text-white cursor-pointer"
            }`}
            onClick={handleCopy}
            disabled={isCopied}
          >
            {isCopied ? <CheckIcon /> : <Clipboard />}
          </button>
          <span className={`transition-colors duration-200 ${isCopied ? "text-green-400" : ""}`}>
            {props.dns}
          </span>
        </p>
        {props.responseTime && (
          <p className="text-xs text-gray-400 text-left ml-4">
            {formatResponseTime(props.responseTime)} { props.errorMessage ? `- ${props.errorMessage}` : "" }
          </p>
        )}
      </div>
      <div className="flex flex-col items-end flex-shrink-0 min-w-0 max-w-[240px]">
        <p
          className={`${
            props.status ? "text-[#3FB950]" : "text-[#F85149]"
          } flex items-center gap-2 mb-1 whitespace-nowrap`}
        >
          {props.status ? "قابل استفاده" : "مسدود شده"}
          <StatusCircle status={props.status} />
        </p>
      </div>
    </div>
  );
}

function CheckIcon() {
  return (
    <svg
      width="16"
      height="16"
      viewBox="0 0 16 16"
      fill="currentColor"
      className="animate-bounce-once"
    >
      <path
        fillRule="evenodd"
        d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"
      />
    </svg>
  );
}

function StatusCircle({ status }: { status: boolean }) {
  return (
    <div className="w-4 h-4">
      <svg
        width="16"
        height="16"
        viewBox="0 0 16 16"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <circle
          cx="8"
          cy="8"
          r="8"
          fill={status ? "url(#greenGradient)" : "url(#redGradient)"}
        />
        <defs>
          {/* Green gradient for true status */}
          <linearGradient
            id="greenGradient"
            x1="0"
            y1="0"
            x2="16"
            y2="16"
            gradientUnits="userSpaceOnUse"
          >
            <stop stopColor="#3FB950" />
            <stop offset="1" stopColor="#13641F" />
          </linearGradient>

          {/* Red gradient for false status */}
          <linearGradient
            id="redGradient"
            x1="0"
            y1="0"
            x2="16"
            y2="16"
            gradientUnits="userSpaceOnUse"
          >
            <stop stopColor="#F85149" />
            <stop offset="1" stopColor="#741611" />
          </linearGradient>
        </defs>
      </svg>
    </div>
  );
}
