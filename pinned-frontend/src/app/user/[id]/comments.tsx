"use client"

import { Comment } from "@/api/comments/dto";
import { Post } from "@/api/post/dto";
import { getPostFromID } from "@/api/post/post";
import { User } from "@/api/user/dto";
import style from "@/components/comments/comments.module.scss";
import LikeChip from "@/components/like-chip/like-chip";
import UserChip from "@/components/user-chip/user-chip";
import Link from "next/link";
import { useEffect, useState } from "react";
import Image from "next/image";
import { CDN_URL } from "@/api/resources";

const Comment = (props: {
  user: User | null,
  comment: Comment
}) => {
  const [parent_post, setParentPost] = useState<Post | null>(null);

  useEffect(() => {
    (async () => {
      const post = await getPostFromID(props.comment.post);
      setParentPost(post);
    })();
  }, []);

  return (
    <div className={style.comment}>
      <div style={{"display": "flex", "alignItems": "center", "gap": "1rem"}}>
        <UserChip user_id={props.comment.creator} />
        {parent_post !== null && 
          <div style={{"backgroundColor": "var(--secondary)", "padding": "0.5rem 1rem", "borderRadius": "2rem"}}>
            <Link href={`/post/${props.comment.post}`} style={{"display": "flex", "alignItems": "center", "gap": "0.5rem"}}>
              Post: {parent_post.title}
              {parent_post.file_id.length >= 1 &&
                <Image 
                  src={CDN_URL + parent_post.file_id}
                  alt="FILE"
                  sizes="100%"
                  width={0}
                  height={0}
                  style={{"width": "2rem", "height": "2rem", "objectFit": "cover", "borderRadius": "50%"}}
                />
              }
            </Link>
          </div>
        }
      </div>
      <p>{props.comment.content}</p>
      <LikeChip dislikes={props.comment.dislikes} likes={props.comment.likes} user={props.user} post_id={props.comment.id} post_type="comment" />
    </div>
  )
}

const UserComments = (props: {
  comments: Comment[] | null,
  user: User | null
}) => {
  if (props.comments === null || props.comments.length <= 0) {
    return (<span>This user has no comments.</span>);
  }

  return (
    <div className={style.comments}>
      {props.comments?.map((comment: Comment, index: number) => {
        return (
          <Comment user={props.user} comment={comment} key={index} />            
        )
      })}
    </div>
  );
}

export default UserComments;
