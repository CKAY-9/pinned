"use client"

import { Post } from "@/api/post/dto"
import { getPostFromID } from "@/api/post/post"
import { User } from "@/api/user/dto"
import LoadingWheel from "@/components/loading/loading"
import { useEffect, useState } from "react"
import preview_style from "@/components/post-preview/post-preview.module.scss";
import PostPreview from "@/components/post-preview/post-preview"
import Link from "next/link"

const FavouritesClient = (props: {
  user: User
}) => {
  const [loading, setLoading] = useState<boolean>(true);
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    (async () => {
      const ps: Post[] = [];
      for (let i = 0; i < props.user.favourites.length; i++) {
        const temp_post = await getPostFromID(props.user.favourites[i]);
        if (temp_post === null) continue;
        ps.push(temp_post);
      }

      setPosts(ps);
      setLoading(false);
    })();
  }, [props.user.favourites])

  if (loading) {
    return (
      <LoadingWheel size_in_rems={2} />
    )
  }

  return (
    <>
      <div className={preview_style.posts} style={{"marginTop": "1rem"}}>
        {posts.map((post, index) => {
          return (
            <PostPreview user={props.user} post={post} key={index}  />
          );
        })}
      </div>
      {posts.length <= 0 && <span>You haven't favourited any posts. We recommend finding some on our <Link href="/post/explore">Explore Page</Link></span>}
    </>
  );
}

export default FavouritesClient;