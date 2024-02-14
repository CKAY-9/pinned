"use client"

import Image from "next/image";
import { useEffect, useState } from "react";
import style from "./theme.module.scss";
import { getTheme } from "./theme-handler";

const ThemeButton = () => {
  const [current_theme, setCurrentTheme] = useState<number | undefined>(undefined);

  useEffect(() => {
    if (current_theme === undefined) {
      setCurrentTheme(getTheme());
    }

    switch (current_theme) {
      case 0:
        window.localStorage.setItem("theme", "dark");
        break;
      case 1:
        window.localStorage.setItem("theme", "light");
        break;
    }
  }, [current_theme]);

  return (
    <>
      {current_theme === 0 &&
        <button onClick={() => setCurrentTheme(1)} className={style.theme_button}>
          <Image src="/icons/lightmode.svg" alt="Lightmode" sizes="100%" width={0} height={0} />
        </button>
      }
      {current_theme === 1 &&
        <button onClick={() => setCurrentTheme(0)} className={style.theme_button}>
          <Image src="/icons/darkmode.svg" alt="Lightmode" sizes="100%" width={0} height={0} />
        </button>
      }
    </>
  );
}

export default ThemeButton;
