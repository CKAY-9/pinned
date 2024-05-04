"use client";

import { User } from "@/api/user/dto";
import explore_style from "@/components/explore-post/explore-post.module.scss";
import style from "./home.module.scss";
import { useEffect, useState } from "react";
import { Post } from "@/api/post/dto";
import { getPinnedPosts } from "@/api/post/post.client";
import ExplorePost from "@/components/explore-post/explore-post";

const HomeClient = (props: { user: User | null }) => {
  const [pinned_posts, setPinnedPosts] = useState<Post[]>([]);

  useEffect(() => {
    (async () => {
      const posts = await getPinnedPosts();
      setPinnedPosts(posts);
    })();
  }, []);

  return (
    <>
      <div className={style.pinned}>
        <div
          className={explore_style.item}
          style={{
            padding: "1rem",
          }}
        >
          <h1 className={style.pinned_splash}>Today&apos;s Pinned</h1>
          <span>View the top posts of today.</span>
        </div>
        {pinned_posts.length <= 0 ? (
          <div
            className={explore_style.item}
            style={{
              backgroundColor: "var(--primary)",
              color: "white",
              width: "100%",
              padding: "1rem",
            }}
          >
            <h1 className={style.pinned_splash}>NO POSTS FOUND</h1>
            <span>It looks like no one has posted anything in the past 24hrs.</span>
          </div>
        ) : (
          <>
            {pinned_posts.map((post, index) => {
              return (
                <ExplorePost post={post} key={index} />
              )
            })}
          </>
        )}
      </div>
    </>
  );
};

export default HomeClient;
