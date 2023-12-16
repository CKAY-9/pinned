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
    console.log(body_height / window_height);
    if (body_height / window_height < 0.7) {
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
