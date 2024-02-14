"use client"

import { useEffect, useState } from "react";

export const getTheme = () => {
  let selected_theme = window.localStorage.getItem("theme");
  if (selected_theme === null) {
    window.localStorage.setItem("theme", "dark");
    selected_theme = "dark";
  }

  switch (selected_theme) {
    case "dark":
      return 0;
      break;
    case "light":
      return 1;
      break;
  }
  return 0;
}

const ThemeHandler = () => {
  const [theme, setTheme] = useState<number>(0);

  useEffect(() => {
    setInterval(() => {
      setTheme(getTheme());
    }, 100);
  }, []);

  return (
    <>
      {theme === 0 &&
        <style jsx global>{
          `
            :root {
              --text: #f0eff6;
              --background: #090813;
              --primary: #948ade;
              --secondary: #2a1b95;
              --accent: #3117eb;
            }
          `
        }</style>
      }
      {theme === 1 &&
        <style jsx global>{
        `
          :root {
            --text: #080811;
            --background: #f8f8fc;
            --primary: #5650c4;
            --secondary: #6259FF;
            --accent: #665fd9;
          }

          footer, header, input, button, textarea {
            color: var(--background) !important;
            * {
              color: var(--background) !important;
            }
          }
        `
        }</style>
      }
    </>
  );
}

export default ThemeHandler;
