"use client";

import { User } from "@/api/user/dto";
import style from "./user.module.scss";
import { useEffect, useState } from "react";
import { usePathname, useSearchParams, useRouter } from "next/navigation";

export const UserCreations = (props: {
  profile: User
}) => {
  const params = useSearchParams();
  const router = useRouter();
  const pathname = usePathname();

  const [current_view, setCurrentView] = useState<number>(0);

  useEffect(() => {
    const view = params.get("view");
    if (view != null) {
      switch (view.toLowerCase()) {
        case "posts":
          setCurrentView(0);
          break;
        case "collections":
          setCurrentView(1);
          break;
        case "comments":
          setCurrentView(2);
          break;
      }
    }
  }, []);

  const changeView = (view: {index: number, view: string}) => {
    setCurrentView(view.index);
    router.push(pathname + "?" + "view=" + view.view);
  }

  return (
    <>
      <section className={style.nav_buttons}>
        <button onClick={() => changeView({index: 0, view: "posts"})} style={{"backgroundColor": current_view === 0 ? "var(--primary)" : "transparent"}}>Posts</button>
        <button onClick={() => changeView({index: 1, view: "collections"})} style={{"backgroundColor": current_view === 1 ? "var(--primary)" : "transparent"}}>Collections</button>
        <button onClick={() => changeView({index: 2, view: "comments"})} style={{"backgroundColor": current_view === 2 ? "var(--primary)" : "transparent"}}>Comments</button>
      </section>
    </>
  );
}

export default UserCreations;
