"use client"

import { Collection } from "@/api/collections/dto";
import UserChip from "@/components/user-chip/user-chip";
import style from "./collection.module.scss";
import posts_style from "@/components/post-preview/post-preview.module.scss";
import { BaseSyntheticEvent, useEffect, useState } from "react";
import { Post } from "@/api/post/dto";
import { getPostFromID } from "@/api/post/post";
import { deleteCollection } from "@/api/collections/collections.client";
import { createNotification } from "@/utils/notification";
import PostPreview from "@/components/post-preview/post-preview";

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
          <PostPreview post={post} key={index} />
        )
      })}
    </div>
  );
}

const CollectionClient = (props: {
  collection: Collection
}) => {
  const deleteColl = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const deletion = await deleteCollection(props.collection.id);
    if (deletion === null) {
      createNotification("Failed to delete collection.");
      return;
    }
    createNotification("Deleted collection!");
    window.location.href = `/user/${props.collection.creator}?view=collections`;
  }

  return (
    <>
      <div className={style.collection_header}>
        <h1>{props.collection.name}</h1>
        <p>{props.collection.description}</p>
        <div style={{"display": "flex", "gap": "1rem"}}>
          <UserChip user_id={props.collection.creator} />
          <button className="impact" onClick={deleteColl}>Delete Collection</button>
        </div>
      </div>
      <div>
        <Posts posts={props.collection.linked_posts} />
      </div>
    </>
  );
}

export default CollectionClient;
