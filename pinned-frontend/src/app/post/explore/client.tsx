"use client";

import { useEffect, useState } from "react";
import style from "./explore.module.scss";
import { User } from "@/api/user/dto";
import { getExploreUsers } from "@/api/user/user.client";
import Link from "next/link";
import { getExplorePosts } from "@/api/post/post.client";
import { Post } from "@/api/post/dto";
import { CDN_URL } from "@/api/resources";
import LikeChip from "@/components/like-chip/like-chip";

const ExplorePost = (props: { post: Post }) => {
  return (
    <Link
      href={`/post/${props.post.id}`}
      className={style.item}
      style={{
        background: `url(${CDN_URL + props.post.file_id})`,
        color: "white",
        backgroundSize: "cover",
        backgroundPositionX: "50%",
      }}
    >
      <div
        className={style.content}
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        <section>
          <h1 style={{ fontSize: "4rem" }}>{props.post.title}</h1>
          <span>{props.post.description}</span>
        </section>
        <section>
          <LikeChip
            likes={props.post.likes}
            dislikes={props.post.dislikes}
            post_id={props.post.id}
            post_type="post"
            user={null}
          ></LikeChip>
        </section>
      </div>
    </Link>
  );
};

const ExplorePostsClient = () => {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      const ps = await getExplorePosts();
      setPosts((old) => old.concat(ps));
      console.log(ps);
      setLoading(false);
    })();
  }, []);

  return (
    <div className={style.explore}>
      <div className={style.item} style={{ padding: "1rem" }}>
        <h1 style={{ fontSize: "4rem" }}>EXPLORE POSTS</h1>
        <span>Find posts on Pinned and explore them.</span>
      </div>
      {loading && (
        <div
          className={style.item}
          style={{
            backgroundColor: "var(--primary)",
            color: "white",
            padding: "1rem",
          }}
        >
          <h1 style={{ fontSize: "4rem" }}>LOADING...</h1>
          <span>Please wait while we load some users for you...</span>
        </div>
      )}
      {posts.length <= 0 && !loading && (
        <div
          className={style.item}
          style={{
            backgroundColor: "var(--primary)",
            color: "white",
            padding: "1rem",
          }}
        >
          <h1 style={{ fontSize: "4rem" }}>NO POSTS FOUND</h1>
          <span>Failed to get any posts, strange...</span>
        </div>
      )}
      {posts.length >= 1 && !loading && (
        <>
          {posts.map((val, index) => {
            return <ExplorePost key={index} post={val}></ExplorePost>;
          })}
        </>
      )}
    </div>
  );
};

export default ExplorePostsClient;
