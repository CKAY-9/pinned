"use client"

import Image from "next/image";
import style from "./more-menu.module.scss";
import { useState } from "react";

const MoreMenu = (props: {
  children: any
}) => {
  const [show_menu, setShowMenu] = useState<boolean>(false);

  return (
    <div className={style.container}>
      <button onClick={() => setShowMenu(!show_menu)} className={style.more_button} style={{"opacity": show_menu ? "1" : "0.5"}}>
        <Image 
          src="/icons/more.svg"
          alt="More Menu"
          sizes="100%"
          width={0}
          height={0}
          className={style.more}
        />
      </button>
      {show_menu && (
        <div className={style.menu}>
          {props.children}
        </div>
      )}
    </div>
  );
}

export default MoreMenu;