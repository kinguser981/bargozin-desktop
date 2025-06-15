interface SpinnerProps {
  size?: "sm" | "md" | "lg" | "xl";
  color?: "primary" | "secondary" | "white" | "gray";
  className?: string;
}

const sizeClasses = {
  sm: "w-4 h-4 border-2",
  md: "w-6 h-6 border-2",
  lg: "w-8 h-8 border-3",
  xl: "w-12 h-12 border-4",
};

const colorClasses = {
  primary: "border-blue-200 border-t-blue-600",
  secondary: "border-purple-200 border-t-purple-600",
  white: "border-gray-300 border-t-white",
  gray: "border-gray-200 border-t-gray-600",
};

export default function Spinner(props: { className?: string }) {
  return (
    <div
      className={`flex items-center justify-center relative w-24 h-24 ${props.className}`}
    >
      {/* Outer half circle - bigger, spinning clockwise */}
      <svg
        width="96"
        height="96"
        viewBox="0 0 96 96"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className="absolute spin-clockwise"
      >
        <path
          d="M12 48 A36 36 0 1 1 84 48"
          stroke="url(#paint0_linear_outer)"
          strokeWidth="8"
          strokeLinecap="round"
          fill="none"
        />
        <defs>
          <linearGradient
            id="paint0_linear_outer"
            x1="12"
            y1="48"
            x2="84"
            y2="48"
            gradientUnits="userSpaceOnUse"
          >
            <stop stopColor="#2F81F7" />
            <stop offset="1" stopColor="#1C4C91" />
          </linearGradient>
        </defs>
      </svg>

      {/* Inner half circle - smaller, spinning counter-clockwise */}
      <svg
        width="64"
        height="64"
        viewBox="0 0 64 64"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className="absolute spin-counter-clockwise"
      >
        <path
          d="M52 32 A20 20 0 1 1 12 32"
          stroke="#96989A"
          strokeWidth="6"
          strokeLinecap="round"
          fill="none"
        />
      </svg>
    </div>
  );
}
