"use client"

import { useEffect } from "react";

const FooterClient = () => {
  useEffect(() => {
    if (typeof(document) === undefined && typeof(window) === undefined) return;

    const body = document.body;
    const footer = document.getElementById("footer_main");
    if (body === null || footer === null) return;

    const body_height = body.scrollHeight;
    const window_height = window.innerHeight;
    if (body_height / window_height < 0.85) {
      footer.style.position = "fixed";
      footer.style.bottom = "0";
      footer.style.width = "100%";
    } else {
      footer.style.position = "relative";
    }

  }); 

  return (
    <>
    </>
  )
}

export default FooterClient;
