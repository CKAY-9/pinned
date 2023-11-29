"use client";

import { setCookie } from "@/utils/cookies";
import { useSearchParams } from "next/navigation";

const LoginClient = () => {
  const searchParams = useSearchParams();
  const token = searchParams.get("token");
  if (token !== null) {
    setCookie("token", token, 365);
    window.location.href = "/"; 
  }

  return (
    <>
    </>
  );
}

export default LoginClient;
