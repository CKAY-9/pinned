"use client"

import Link from "next/link";
import Image from "next/image";
import style from "./post-preview.module.scss";
import { Post } from "@/api/post/dto";
import { CDN_URL } from "@/api/resources";
import MoreMenu from "../more-menu/more-menu";
import PinPostButton from "../pin/pin";
import { User } from "@/api/user/dto";
import FavouriteButton from "../favourite/favourite";
import { BaseSyntheticEvent } from "react";
import { deletePost } from "@/api/post/post.client";

const PostPreview = (props: { post: Post; pinned?: boolean; user?: User | null}) => {
  const deleteP = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const d = await deletePost(props.post.id);
    if (d !== null) {
      window.location.reload();
    }
  }
  
  return (
    <>
      <div className={style.post}>
        <section className={style.info}>
          <h1>{props.post.title}</h1>
          <MoreMenu>
            <>
              {(props.user !== null && props.user !== undefined) && 
                <>
                  {props.user.id === props.post.creator && (
                    <>
                      <PinPostButton post_id={props.post.id} user={props.user}></PinPostButton>
                      <button className="impact" onClick={deleteP}>Delete</button>
                    </>
                  )}
                  <FavouriteButton user={props.user} post_id={props.post.id}></FavouriteButton>
                </>
              }
            </>
          </MoreMenu>
          {props.pinned && (
            <Image
              src="/icons/star.svg"
              alt="Pinned"
              sizes="100%"
              width={36}
              height={36}
            />
          )}
        </section>
        <Link href={`/post/${props.post.id}`}>
          <Image
            src={(CDN_URL + props.post.file_id) || "/icons/loading.svg"}
            className={style.preview}
            alt="Post Picture"
            sizes="100%"
            width={0}
            height={0}
          />
        </Link>
      </div>
    </>
  );
};

export default PostPreview;
