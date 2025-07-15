import { openUrl } from "@tauri-apps/plugin-opener";
import Linkedin from "./svg/linkedin";

export default function PeopleItem(props: {
  image: string;
  name: string;
  role: string;
  linkedinUrl?: string;
}) {
  const handleLinkedInClick = async (e: React.MouseEvent) => {
    e.preventDefault();
    if (props.linkedinUrl) {
      try {
        await openUrl(props.linkedinUrl);
      } catch (error) {
        console.error('Failed to open LinkedIn URL:', error);
      }
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-w-[250px]">
      <div className="w-[80px] h-[80px] rounded-full bg-gradient-to-br from-[#2F81F7] to-[#1C4C91] p-[4px]">
        <img
          src={props.image}
          alt={props.name}
          className="w-full h-full rounded-full object-cover"
          onError={(e) => {
            e.currentTarget.src = "/profile.png";
          }}
        />
      </div>
      <div className="flex flex-col items-center justify-center mt-5">
        <h4 className="text-sm font-bold text-center">
          {props.name} <span className="text-xs text-[#CDCDCD] font-thin">{props.role}</span>
        </h4>
        {props.linkedinUrl && (
          <button 
            onClick={handleLinkedInClick}
            className="flex items-center gap-2 text-xs font-normal underline cursor-pointer hover:text-blue-600 transition-colors"
          >
            پروفایل لینکدین
            <Linkedin />
          </button>
        )}
      </div>
    </div>
  );
}
