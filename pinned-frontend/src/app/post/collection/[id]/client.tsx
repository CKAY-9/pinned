"use client"

import { Collection } from "@/api/collections/dto";
import UserChip from "@/components/user-chip/user-chip";
import style from "./collection.module.scss";
import posts_style from "@/app/user/[id]/posts.module.scss";
import { useEffect, useState } from "react";
import { Post } from "@/api/post/dto";
import { getPostFromID } from "@/api/post/post";
import Link from "next/link";

const Posts = (props: {
  posts: number[]
}) => {
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    (async () => {
      for (let i = 0; i < props.posts.length; i++) {
        const post = await getPostFromID(props.posts[i]);
        if (post === null) continue;
        setPosts((prevPosts) => [
          ...prevPosts.filter((p) => p.id != post.id),
          post
        ]);
      }
    })();
  }, []);

  return (
    <div className={posts_style.posts}>
      {posts.map((post: Post, index: number) => {
        return (
          <Link key={index} href={`/post/${post.id}`} className={posts_style.post}>
            <h1>{post.title}</h1>
            <p>{post.description.substring(0, 50)}</p>
          </Link>
        )
      })}
    </div>
  );
}

const CollectionClient = (props: {
  collection: Collection
}) => {
  return (
    <>
      <div className={style.collection_header}>
        <h1>{props.collection.name}</h1>
        <p>{props.collection.description}</p>
        <UserChip user_id={props.collection.creator} />
      </div>
      <div>
        <Posts posts={props.collection.linked_posts} />
      </div>
    </>
  );
}

export default CollectionClient;
