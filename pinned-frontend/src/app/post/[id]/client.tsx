"use client"

import { Post } from "@/api/post/dto"
import { User } from "@/api/user/dto"
import style from "./post.module.scss";
import Image from "next/image";
import { BaseSyntheticEvent, useEffect, useState } from "react";
import { getUserCollections } from "@/api/user/user.client";
import UserChip from "@/components/user-chip/user-chip";
import Popup from "@/components/popup/popup";
import { Collection } from "@/api/collections/dto";
import { addToCollection, newCollection } from "@/api/collections/collections.client";
import { createNotification } from "@/utils/notification";
import { deletePost } from "@/api/post/post.client";
import { CDN_URL } from "@/api/resources";
import LikeChip from "@/components/like-chip/like-chip";
import Comments from "@/components/comments/comments";
import { createComment } from "@/api/comments/comment.client";

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
    <div style={{"position": "relative"}}>
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
      <div className={style.post_post}>
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
  const [show_new_comment, setShowNewComment] = useState<boolean>(false);
  const [new_comment_content, setNewCommentContent] = useState<string>("");

  useEffect(() => {
    (async () => {
      const collections = await getUserCollections(props.user?.id || 0);
      setMyCollections(collections);
    })();
  }, []);

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

  const newComment = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const creation = await createComment(new_comment_content, props.post.id);
    if (creation !== null) {
      window.location.reload();
      return;
    }
  }

  return (
    <>
      {(popup && props.user !== null) && 
        <Popup>
          <button onClick={() => setPopup(false)}>X</button>
          <div style={{"display": "flex", "flexDirection": "column", "gap": "1rem"}}>
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
      {(show_new_comment && props.user !== null) &&
        <Popup>
          <button style={{"backgroundColor": "transparent"}} onClick={() => setShowNewComment(false)}>X</button>
          <h1>New Comment</h1>
          <label>Comment</label>
          <textarea rows={10} cols={30} placeholder="Your Comment"  onChange={(e: BaseSyntheticEvent) => setNewCommentContent(e.target.value)} />
          <button onClick={newComment}>Post</button>
        </Popup>
      }
      <div className={style.post_container}>
        <h1>{props.post.title}</h1>
        {props.post.file_id.length >= 1 &&
          <PostPicture post_title={props.post.title} pic_url={CDN_URL + props.post.file_id} />
        }
        <p>{props.post.description}</p>
        <LikeChip user={props.user} post_id={props.post.id} likes={props.post.likes} dislikes={props.post.dislikes} post_type={"post"} />
        <section className={style.post_interaction}>
          <UserChip user_id={props.post.creator} />
          {props.user !== null && <button onClick={() => setPopup(!popup)} className="impact">Add to Collection</button>}
          {is_owner && <button className="impact" onClick={deleteP}>Delete Post</button>
          }
        </section>
        <Comments comments_only={false} post_id={props.post.id} user={props.user} comment_ids={props.post.comments} />
      </div>
      <div>
        <h2>More Posts Like This</h2>
      </div>
    </>
  );
}

export default PostClient;
