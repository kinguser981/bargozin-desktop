import { openUrl } from "@tauri-apps/plugin-opener";
import PeopleItem from "../components/people-item";
import Github from "../components/svg/github";
import Linkedin from "../components/svg/linkedin";
import Team from "../components/svg/team";
import ArrowLeft from "../components/svg/arrow-left";
import { Link } from "react-router";

export default function About() {
  const handleLinkedInClick = async (url: string) => {
    try {
      await openUrl(url);
    } catch (error) {
      console.error("Failed to open LinkedIn URL:", error);
    }
  };

  const team = [
    {
      name: "زهرا محمدی",
      role: "طراح محصول",
      linkedinUrl: "https://www.linkedin.com/in/zhrwmmdi/",
      image: "https://media.licdn.com/dms/image/v2/D4D03AQHMcokiDxvy9A/profile-displayphoto-shrink_400_400/B4DZOA_NEmHgAg-/0/1733035873156?e=1758153600&v=beta&t=jN_6D8tPC9feFPQku1Wx3T6jfmR4zTc7yr0xMfFsMuA",
    },
    {
      name: "وصال ژولانژاد",
      role: "توسعه دهنده",
      linkedinUrl: "https://www.linkedin.com/in/vesal-joolanejad/",
      image: "https://media.licdn.com/dms/image/v2/D4D03AQH1Qz4sc2C4gw/profile-displayphoto-scale_400_400/B4DZgMFP6pHwAg-/0/1752549360004?e=1758153600&v=beta&t=Pe_E1RDaLvGrPOqwhF7AZFL0mTVjKd2WjASHO44Gmhk",
    },
  ];

  return (
    <div>
      <h1 className="text-2xl font-bold text-center">درباره ما</h1>

      <div className="bg-[#161B22] rounded-lg p-4 text-right text-md mt-5">
        <p className="mb-5">
          ما جمعی از افراد متخصص در زمینه‌های مختلف زمینه نرم‌افزار و توسعه
          محصول هستیم که با توجه به محدودیت‌های موجود در دسترسی به بسیاری از
          سرویس‌ها در ایران، تصمیم گرفتیم ابزاری طراحی کنیم که به کاربران این
          امکان را بدهد تا راحت‌تر به تکنولوژی‌های مورد نیاز خود دسترسی پیدا
          کنند.{" "}
        </p>
        <p>
          هدف اصلی ما، ارائه راه‌حلی کارآمد برای تسهیل دسترسی به منابع جهانی و
          هموار کردن مسیر پیشرفت برای کاربران ایرانی است. امیدواریم با گسترش و
          بهبود مستمر این محصول، به یکی از راهکارهای مؤثر برای رفع محدودیت‌ها
          برای هم‌وطنانمان تبدیل شویم.
        </p>
      </div>

      <div className="mt-5">
        <h1 className="text-2xl font-bold text-center flex items-center gap-2 justify-center mb-5">
          تیم ما
          <Team className="scale-75" />
        </h1>
        <div className="flex justify-center">
          {team.map((person, index) => (
            <PeopleItem
              key={index}
              image={person.image}
              name={person.name}
              role={person.role}
              linkedinUrl={person.linkedinUrl}
            />
          ))}
        </div>

        <div className="mt-5">
          <p className="text-center text-sm">
            اگر علاقه‌مند به مشارکت در توسعه این ابزار هستید یا پیشنهادی برای
            بهبود آن دارید، خوشحال می‌شویم از طریق گیت‌هاب یا لینکدین با ما در
            ارتباط باشید.
          </p>

          <div className="flex justify-center items-center gap-2 mt-5 underline">
            <div
              className="flex justify-center items-center gap-2 cursor-pointer"
              onClick={() =>
                handleLinkedInClick(
                  "https://github.com/403unlocker/bargozin-desktop"
                )
              }
            >
              <Github />
              مشاهده سورس کد پروژه
            </div>
            <div className="flex justify-center items-center gap-2 cursor-pointer">
              <Linkedin />
              مشاهده لینکدین برگُزین
            </div>
          </div>
        </div>
        <Link
          to="/"
          className="text-center text-sm mt-5 flex justify-center items-center gap-2 cursor-pointer"
        >
          <ArrowLeft />
          بازگشت به سرویس‌ها
        </Link>
      </div>
    </div>
  );
}
