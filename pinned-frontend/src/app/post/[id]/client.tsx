"use client"

import { Post } from "@/api/post/dto"
import { User } from "@/api/user/dto"
import style from "./post.module.scss";
import Image from "next/image";
import { useEffect, useState } from "react";
import { getUserFromID } from "@/api/user/user.client";
import LoadingWheel from "@/components/loading/loading";
import Link from "next/link";

const PostClient = (props: {
  post: Post,
  user: User | null
}) => {
  const [creator, setCreator] = useState<null | User>(null);
  const [loading_creator, setLoadingCreator] = useState<boolean>(true);
  const is_owner = props.user?.id == props.post.creator;
    
  useEffect(() => {
    (async () => {
      const get_creator = await getUserFromID(props.post.creator);
      setCreator(get_creator);
      setLoadingCreator(false);
    })();
  }, [props.post.creator]);

  return (
    <>
      <div className={style.post_container}>
        <h1>{props.post.title}</h1>
        <section className={style.interaction}>
          <button className={style.like}>
            <Image
              src="/icons/like.svg"
              alt="Likes"
              sizes="100%"
              width={0}
              height={0}
              style={{"opacity": props.post.likes.includes(props.user?.id || 0) ? 1 : 0.5}}
            />
            <span>{props.post.likes.length}</span>
          </button>
          <button className={style.like}>
            <Image
              src="/icons/dislike.svg"
              alt="Likes"
              sizes="100%"
              width={0}
              height={0}
              style={{"opacity": props.post.likes.includes(props.user?.id || 0) ? 1 : 0.5}}
            />
            <span>{props.post.dislikes.length}</span>
          </button>
        </section>
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
        <p>{props.post.description}</p>
      </div>
      <div>
        <h2>More Posts Like This</h2>
      </div>
    </>
  );
}

export default PostClient;
