"use client"

import { Post } from "@/api/post/dto"
import { User } from "@/api/user/dto"
import style from "./post.module.scss";
import Image from "next/image";
import { BaseSyntheticEvent, useEffect, useState } from "react";
import { getUserCollections, getUserFromID } from "@/api/user/user.client";
import UserChip from "@/components/user-chip/user-chip";
import Popup from "@/components/popup/popup";
import { Collection } from "@/api/collections/dto";
import { addToCollection } from "@/api/collections/collections.client";
import { createNotification } from "@/utils/notification";

const AddToCollection = (props: {
  collection: Collection,
  post_id: number
}) => {
  const add = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const add_response = await addToCollection(props.collection.id, props.post_id);
    if (add_response !== null) {
      createNotification("Updated collection");
      window.location.href = `/post/collection/${props.collection.id}`;
      return;
    }
  }

  return (
    <button onClick={add}>
      <span>{props.collection.name}</span>
      <span style={{"marginLeft": "1rem"}}>Entries: {props.collection.linked_posts.length + props.collection.linked_comments.length}</span>
    </button>
  )
}

const PostClient = (props: {
  post: Post,
  user: User | null
}) => {
  const is_owner = props.user?.id == props.post.creator;
  const [popup, setPopup] = useState<boolean>(false);
  const [my_collections, setMyCollections] = useState<Collection[]>([]);

  useEffect(() => {
    (async () => {
      const collections = await getUserCollections(props.user?.id || 0);
      setMyCollections(collections);
    })();
  }, []);
    
  return (
    <>
      {popup && 
        <Popup>
          <div style={{"display": "flex", "flexDirection": "column", "gap": "1rem"}}>
            <button onClick={() => setPopup(false)} style={{"backgroundColor": "transparent", "padding": "0"}}>X</button>
            <h2>Add to Collection</h2>
            {my_collections.map((collection: Collection, index: number) => {
              return (<AddToCollection post_id={props.post.id} collection={collection} key={index} />)
            })}
          </div>
        </Popup>
      }
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
        <section className={style.post_interaction}>
          <UserChip user_id={props.post.creator} />
          <button onClick={() => setPopup(!popup)} className="impact">Add to Collection</button>
        </section>
        <p>{props.post.description}</p>
      </div>
      <div>
        <h2>More Posts Like This</h2>
      </div>
    </>
  );
}

export default PostClient;
