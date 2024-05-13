"use client";

import { User } from "@/api/user/dto";
import style from "./user.module.scss";
import { useEffect, useState } from "react";
import { usePathname, useSearchParams, useRouter } from "next/navigation";
import { Post } from "@/api/post/dto";
import { getUserCollections, getUserComments, getUserPosts, logoutUser } from "@/api/user/user.client";
import UserPosts from "./posts";
import UserCollections from "./collections";
import UserComments from "./comments";
import Link from "next/link";
import { Collection } from "@/api/collections/dto";
import { Comment } from "@/api/comments/dto";
import { getPostFromID } from "@/api/post/post";

export const UserInteraction = (props: {
  profile: User,
  user: User | null
}) => {
  return (
    <div className={style.user_interaction}>
      <button onClick={logoutUser} className="impact">Logout</button>
      <Link href={`/user/settings`} className="impact">Settings</Link>
    </div>
  );
}

export const UserCreations = (props: {
  profile: User,
  user: User | null
}) => {
  const params = useSearchParams();
  const router = useRouter();
  const pathname = usePathname();

  const [current_view, setCurrentView] = useState<number>(0);
  const [user_posts, setUserPosts] = useState<Post[] | null>(null);
  const [user_collections, setUserCollections] = useState<Collection[] | null>(null);
  const [user_comments, setUserComments] = useState<Comment[] | null>(null);

  useEffect(() => {
    (async () => {
      const pinned_posts: Post[] = [];
      for (let i = 0; i < props.profile.pinned.length; i++) {
        const temp_post = await getPostFromID(props.profile.pinned[i]);
        if (temp_post === null) break;
        pinned_posts.push(temp_post);
      }
      const all_posts: Post[] = await getUserPosts(props.profile.id);
      const posts = pinned_posts.concat(all_posts);
      setUserPosts(posts);

      const collections = await getUserCollections(props.profile.id);
      setUserCollections(collections);

      const comments = await getUserComments(props.profile.id);
      setUserComments(comments);
    })();

    const view = params.get("view");
    if (view != null) {
      switch (view.toLowerCase()) {
        case "posts":
          setCurrentView(0);
          break;
        case "collections":
          setCurrentView(1);
          break;
        case "comments":
          setCurrentView(2);
          break;
      }
    }
  }, [params, props.profile]);

  const changeView = (view: {index: number, view: string}) => {
    setCurrentView(view.index);
    router.push(pathname + "?" + "view=" + view.view);
  }

  const is_user = (props.user !== null && props.user !== undefined) && (props.user.id === props.profile.id);

  return (
    <>
      <section className={style.nav_buttons}>
        <button onClick={() => changeView({index: 0, view: "posts"})} style={{"backgroundColor": current_view === 0 ? "var(--primary)" : "transparent"}}>Posts</button>
        <button onClick={() => changeView({index: 1, view: "collections"})} style={{"backgroundColor": current_view === 1 ? "var(--primary)" : "transparent"}}>Collections</button>
        <button onClick={() => changeView({index: 2, view: "comments"})} style={{"backgroundColor": current_view === 2 ? "var(--primary)" : "transparent"}}>Comments</button>
      </section>
      <section>
        <div style={{"display": current_view === 0 ? "flex" : "none"}} className={style.creation}>
          {is_user && <>
            <Link className={`${style.interact} impact`} href="/post/new">New Post</Link> 
          </>
          }
          <UserPosts posts={user_posts} />
        </div>
        <div style={{"display": current_view === 1 ? "flex" : "none"}} className={style.creation}>
          {is_user && <>
            <Link className={`${style.interact} impact`} href="/post/collection/new">New Collection</Link> 
          </>
          }
          <UserCollections collections={user_collections} />
        </div>
        <div style={{"display": current_view === 2 ? "flex" : "none"}} className={style.creation}>  
          <UserComments user={props.user} comments={user_comments} />
        </div>
      </section>
    </>
  );
}

export default UserCreations;
