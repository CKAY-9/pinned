"use client";

import { useEffect, useState } from "react";
import style from "./explore.module.scss";
import { getExplorePosts } from "@/api/post/post.client";
import { Post } from "@/api/post/dto";
import ExplorePost from "@/components/explore-post/explore-post";

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
