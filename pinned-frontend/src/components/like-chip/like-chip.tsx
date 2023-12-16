"use client";

import Image from "next/image";
import style from "./like-chip.module.scss";
import { BaseSyntheticEvent, useState } from "react";
import { User } from "@/api/user/dto";
import { likePost } from "@/api/post/post.client";

const LikeChip = (props: {
  likes: number[],
  dislikes: number[],
  post_id: number,
  user: User | null,
  post_type: "post" | "collection" | "comment"
}) => {
  const [likes, setLikes] = useState<number[]>(props.likes);
  const [dislikes, setDislikes] = useState<number[]>(props.dislikes);

  const like = async (e: BaseSyntheticEvent, like_type: number) => {
    e.preventDefault();
    if (props.user === null) return;

    switch (props.post_type) {
      case "post":
        await likePost(like_type, props.post_id);
        break;
      case "collection":
        break;
      case "comment":
        break;
    }

    if (like_type === 1) {
      if (likes.includes(props.user.id)) {
        setLikes((likes) => likes.filter((id) => id !== props.user?.id || 0));
        return;
      }
      if (dislikes.includes(props.user.id)) {
        setDislikes((dislikes) => dislikes.filter((id) => id !== props.user?.id || 0));
      }
      setLikes((ids) => [
        ...ids,
        props.user?.id || 0
      ]);
      return;
    }
    if (like_type === -1) {
      if (dislikes.includes(props.user.id)) {
        setDislikes((dislikes) => dislikes.filter((id) => id !== props.user?.id || 0));
        return;
      }
      if (likes.includes(props.user.id)) {
        setLikes((likes) => likes.filter((id) => id !== props.user?.id || 0));
      }
      setDislikes((ids) => [
        ...ids,
        props.user?.id || 0
      ]);
      return;
    }
  }

  return (
    <>
      <section className={style.like_chip}>
        <button onClick={async (e: BaseSyntheticEvent) => {
          await like(e, 1);
        }} className={style.like}>
          <Image
            src="/icons/like.svg"
            alt="Likes"
            sizes="100%"
            width={0}
            height={0}
            style={{"opacity": likes.includes(props.user?.id || 0) ? 1 : 0.5}}
          />
          <span>{likes.length}</span>
        </button>
        <button onClick={async (e: BaseSyntheticEvent) => {
          await like(e, -1);
        }} className={style.like}>
          <Image
            src="/icons/dislike.svg"
            alt="Likes"
            sizes="100%"
            width={0}
            height={0}
            style={{"opacity": dislikes.includes(props.user?.id || 0) ? 1 : 0.5}}
          />
          <span>{dislikes.length}</span>
        </button>
      </section>
    </>
  );
}

export default LikeChip;
