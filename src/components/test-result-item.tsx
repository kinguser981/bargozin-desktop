import Clipboard from "./svg/clipboard";

export default function TestResultItem(props: {
  dns: string;
  status: boolean;
  responseTime?: number;
  errorMessage?: string;
}) {
  const formatResponseTime = (time?: number) => {
    if (time === undefined) return "";
    return time < 1000 ? `${time}ms` : `${(time / 1000).toFixed(1)}s`;
  };

  return (
    <div
      className={`${
        props.status ? "bg-[#142A20]" : "bg-[#301B1F]"
      } h-[70px] rounded-lg mb-2 flex justify-between items-center px-5`}
    >
      <div className="flex flex-col">
        <p className="flex items-center mb-1">
          <button
            className="cursor-pointer ml-2"
            onClick={() => navigator.clipboard.writeText(props.dns)}
          >
            <Clipboard />
          </button>
          {props.dns}
        </p>
        {props.responseTime && (
          <p className="text-xs text-gray-400">
            {formatResponseTime(props.responseTime)}
          </p>
        )}
      </div>
      <div className="flex flex-col items-end">
        <p className={`${props.status ? "text-[#3FB950]" : "text-[#F85149]"} flex items-center gap-2 mb-1`}>
          {props.status ? "قابل استفاده" : "مسدود شده"}
          <StatusCircle status={props.status} />
        </p>
        {!props.status && props.errorMessage && (
          <p className="text-xs text-gray-400 text-right max-w-[150px] truncate" title={props.errorMessage}>
            {props.errorMessage}
          </p>
        )}
      </div>
    </div>
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
