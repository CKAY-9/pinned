"use client";

import { User } from "@/api/user/dto";
import style from "./user.module.scss";
import { useState } from "react";

export const UserCreations = (props: {
  profile: User
}) => {
  const [current_view, setCurrentView] = useState<number>(0);

  return (
    <>
      <section className={style.nav_buttons}>
        <button onClick={() => setCurrentView(0)} style={{"backgroundColor": current_view === 0 ? "var(--primary)" : "transparent"}}>Posts</button>
        <button onClick={() => setCurrentView(1)} style={{"backgroundColor": current_view === 1 ? "var(--primary)" : "transparent"}}>Collections</button>
        <button onClick={() => setCurrentView(2)} style={{"backgroundColor": current_view === 2 ? "var(--primary)" : "transparent"}}>Comments</button>
      </section>
    </>
  );
}

export default UserCreations;
