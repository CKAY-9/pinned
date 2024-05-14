"use client";

import { createComment, getAllCommentsFromIds, getCommentFromID } from "@/api/comments/comment.client";
import { BaseSyntheticEvent, useEffect, useState } from "react";
import style from "./comments.module.scss";
import { User } from "@/api/user/dto";
import UserChip from "../user-chip/user-chip";
import { Comment } from "@/api/comments/dto";
import Image from "next/image";
import Popup from "../popup/popup";
import { createNotification } from "@/utils/notification";
import LikeChip from "../like-chip/like-chip";

const UserComment = (props: {
  comment: Comment,
  user: User | null
  index: number
}) => {
  return (
    <div className={style.comment} style={{"animationDelay": `${200 * props.index}ms`}}>
      <UserChip user_id={props.comment.creator} />
      <p>{props.comment.content}</p>
      <LikeChip dislikes={props.comment.dislikes} likes={props.comment.likes} user={props.user} post_id={props.comment.id} post_type="comment" />
    </div>
  );
}

const Comments = (props: {
  comment_ids: number[]
  user: User | null,
  post_id: number,
  comments_only: boolean
}) => {
  const [show_comments, setShowComments] = useState<boolean>(props.comments_only);
  const [comments, setComments] = useState<Comment[]>([]);
  const [show_new_comment, setShowNewComment] = useState<boolean>(false);
  const [new_comment_content, setNewCommentContent] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      const _comments = await getAllCommentsFromIds(props.comment_ids);
      setComments(_comments);
      setLoading(false);
    })();
  }, [props.comment_ids]);

  const postComment = async (e: BaseSyntheticEvent) => {
    if (props.comments_only || props.user === null) return;
    e.preventDefault();
    const comment = await createComment(new_comment_content, props.post_id);
    if (comment !== null) {
      createNotification("Created comment!");
      setShowNewComment(false);
      window.location.reload();
      return;
    }
    createNotification("Failed to create comment!");
    setShowNewComment(false);
  }

  if (loading) {
    return (<></>);
  }

  return (
    <>
      {(show_new_comment && props.user !== null && !props.comments_only) &&
        <Popup>
          <button onClick={() => setShowNewComment(false)} style={{"mixBlendMode": "difference"}}>X</button>
          <h1>New Comment</h1>
          <textarea rows={10} cols={50} onChange={(e: BaseSyntheticEvent) => setNewCommentContent(e.target.value)} />
          <button onClick={postComment} style={{"width": "fit-content", "marginTop": "1rem"}}>Comment</button>
        </Popup>
      }
      <div className={style.comments_container}>
        <div className={style.comments_header}>
          {!props.comments_only &&
            <>
              {(!loading && comments.length >= 1) ?
                <button onClick={() => setShowComments(!show_comments)} className={style.expand}>
                  <span>Comments</span>
                  <Image 
                    src="/icons/menu_expand.svg"
                    alt="Expand"
                    sizes="100%"
                    width={0}
                    height={0}
                    style={{"transform": show_comments ? "rotate(180deg)" : "rotate(0deg)"}}
                  />
                </button> : <h2>Comments</h2>
              }
              {props.user !== null && <button onClick={() => setShowNewComment(true)} className="impact">New Comment</button>}
            </> 
          }
        </div>
        <div style={{"height": show_comments ? "fit-content" : "0px", "overflow": "hidden"}}>
          <div className={style.comments}>
            {comments.map((comment: Comment, index: number) => {
              return (<UserComment user={props.user} index={index} key={index} comment={comment} />);
            })}
          </div>
        </div>
      </div>
      {(!loading && comments.length <= 0) && <span>No one has posted a comment.</span>}
    </>
  );
}

export default Comments;
