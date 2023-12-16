"use client"

import { Post } from "@/api/post/dto"
import { User } from "@/api/user/dto"
import style from "./post.module.scss";
import Image from "next/image";
import { BaseSyntheticEvent, useEffect, useState } from "react";
import { deleteUser, getUserCollections, getUserFromID } from "@/api/user/user.client";
import UserChip from "@/components/user-chip/user-chip";
import Popup from "@/components/popup/popup";
import { Collection } from "@/api/collections/dto";
import { addToCollection, newCollection } from "@/api/collections/collections.client";
import { createNotification } from "@/utils/notification";
import { deletePost, likePost } from "@/api/post/post.client";
import { CDN_URL } from "@/api/resources";

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

const PostPicture = (props: {
  pic_url: string
  post_title: string
}) => {
  const [expand, setExpand] = useState<boolean>(false);

  return (
    <div style={{"position": "relative", "width": "fit-content"}}>
      {expand &&
        <Popup>
          <button style={{"backgroundColor": "transparent"}} onClick={() => setExpand(false)}>X</button>
          <h1 style={{"textAlign": "center"}}>{props.post_title}</h1>
          <Image 
            src={props.pic_url}
            alt="Post Image"
            sizes="100%"
            width={0}
            height={0}
            className={style.post_image_expanded}
          />
        </Popup>
      }
      <Image 
        src={props.pic_url}
        alt="Post Image"
        sizes="100%"
        width={0}
        height={0}
        className={style.post_image}
      />
      <button className={style.expand_image} onClick={() => setExpand(true)}>
        <Image 
          src="/icons/expand.svg"
          alt="Expand"
          sizes="100%"
          width={0}
          height={0}
        />
      </button>
    </div>
  );
}

const PostClient = (props: {
  post: Post,
  user: User | null
}) => {
  const is_owner = props.user?.id == props.post.creator;
  const [popup, setPopup] = useState<boolean>(false);
  const [my_collections, setMyCollections] = useState<Collection[]>([]);
  const [show_new_collection, setShowNewCollection] = useState<boolean>(false);
  const [new_collection_name, setNewCollectionName] = useState<string>("");

  useEffect(() => {
    (async () => {
      const collections = await getUserCollections(props.user?.id || 0);
      setMyCollections(collections);
    })();
  }, []);

  const like = async (e: BaseSyntheticEvent, like_type: number) => {
    e.preventDefault();
    await likePost(like_type, props.post.id);
  }
 
  const deleteP = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    await deletePost(props.post.id);
    window.location.reload();
  }

  const createNewCollectionAndAddPost = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const create_collection = await newCollection(new_collection_name, "No description provided.");
    if (create_collection === null) {
      setPopup(false);
      createNotification("Failed to create collection");
      return;
    }
    createNotification("Created collection");
    const add_response = await addToCollection(create_collection, props.post.id);
    if (add_response !== null) {
      createNotification("Updated collection");
      window.location.href = `/post/collection/${create_collection}`;
      return;
    }
    createNotification("Failed to update collection");
    setPopup(false);
  }

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
            <button onClick={() => setShowNewCollection(!show_new_collection)}>New Collection</button>
            {show_new_collection && 
              <div style={{"display": "flex", "gap": "0.5rem"}}>
                <input type="text" placeholder="Collection Name" onChange={(e: BaseSyntheticEvent) => setNewCollectionName(e.target.value)} />
                <button onClick={createNewCollectionAndAddPost}>Add</button>
              </div>
            }
          </div>
        </Popup>
      }
      <div className={style.post_container}>
        <h1>{props.post.title}</h1>
        {props.post.file_id.length >= 1 &&
          <PostPicture post_title={props.post.title} pic_url={CDN_URL + props.post.file_id} />
        }
        <p>{props.post.description}</p>
        <section className={style.interaction}>
          <button onClick={async (e: BaseSyntheticEvent) => {
            await like(e, 1);
          }} className={style.like}>
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
          <button onClick={async (e: BaseSyntheticEvent) => {
            await like(e, -1);
          }} className={style.like}>
            <Image
              src="/icons/dislike.svg"
              alt="Likes"
              sizes="100%"
              width={0}
              height={0}
              style={{"opacity": props.post.dislikes.includes(props.user?.id || 0) ? 1 : 0.5}}
            />
            <span>{props.post.dislikes.length}</span>
          </button>
        </section>
        <section className={style.post_interaction}>
          <UserChip user_id={props.post.creator} />
          {props.user !== null && <button onClick={() => setPopup(!popup)} className="impact">Add to Collection</button>}
          {is_owner && <button className="impact" onClick={deleteP}>Delete Post</button>
          }
        </section>
      </div>
      <div>
        <h2>More Posts Like This</h2>
      </div>
    </>
  );
}

export default PostClient;
