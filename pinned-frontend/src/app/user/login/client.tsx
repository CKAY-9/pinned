"use client";

import { getTheme } from "@/components/theme-button/theme-handler";
import { setCookie } from "@/utils/cookies";
import { createNotification } from "@/utils/notification";
import { useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";

const LoginClient = () => {
  const [theme, setTheme] = useState<number>(0)
  const searchParams = useSearchParams();
  const token = searchParams.get("token");
  if (token !== null) {
    createNotification("Logging in...");
    setCookie("token", token, 365);
    window.location.href = "/"; 
  }

  useEffect(() => {
    setInterval(() => {
      setTheme(getTheme());
    }, 250);
  })

  return (
    <>
      {theme === 1 &&
        <style jsx global>
        {
          `
            img {
              background-color: black; 
              padding: 1rem 2rem;
            }
          `
        }
        </style>
      }
    </>
  );
}

export default LoginClient;
