"use client"

import { User } from "@/api/user/dto";
import { getUserFromID } from "@/api/user/user.client";
import Link from "next/link";
import { useEffect, useState } from "react";
import style from "./user-chip.module.scss";
import Image from "next/image";
import LoadingWheel from "../loading/loading";

const UserChip = (props: {
  user_id: number
}) => {
  const [creator, setCreator] = useState<null | User>(null);
  const [loading_creator, setLoadingCreator] = useState<boolean>(true);
  
  useEffect(() => {
    (async () => {
      const get_creator = await getUserFromID(props.user_id);
      setCreator(get_creator);
      setLoadingCreator(false);
    })();
  }, [props.user_id]);

  return (
    <>
      {loading_creator 
        ? <div className={style.creator}>
          <LoadingWheel size_in_rems={2} />
        </div>
        : <Link className={style.creator} href={`/user/${creator?.id || 0}`}>
          <Image 
            src={creator?.avatar || ""}
            alt="Creator PFP"
            sizes="100%"
            width={0}
            height={0}
            className={style.pfp}
          />
          <span>{creator?.username || ""}</span>
        </Link>
      }
    </>
  );
}

export default UserChip;
