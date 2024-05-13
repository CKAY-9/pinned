"use client";

import { Collection } from "@/api/collections/dto";
import UserChip from "@/components/user-chip/user-chip";
import style from "./collection.module.scss";
import posts_style from "@/components/post-preview/post-preview.module.scss";
import { BaseSyntheticEvent, useEffect, useState } from "react";
import { Post } from "@/api/post/dto";
import { getPostFromID } from "@/api/post/post";
import {
  addCollaboratorToCollection,
  deleteCollection,
  getCollectionCollaborators,
  removeCollaboratorFromCollection,
} from "@/api/collections/collections.client";
import { createNotification } from "@/utils/notification";
import PostPreview from "@/components/post-preview/post-preview";
import LikeChip from "@/components/like-chip/like-chip";
import { User } from "@/api/user/dto";
import Popup from "@/components/popup/popup";
import { searchUsers } from "@/api/user/user.client";
import UserPreview from "@/components/user-preview/preview";

const Posts = (props: { posts: number[] }) => {
  const [posts, setPosts] = useState<Post[]>([]);

  useEffect(() => {
    (async () => {
      for (let i = 0; i < props.posts.length; i++) {
        const post = await getPostFromID(props.posts[i]);
        if (post === null) continue;
        setPosts((prevPosts) => [
          ...prevPosts.filter((p) => p.id != post.id),
          post,
        ]);
      }
    })();
  }, [props.posts]);

  if (posts.length <= 0) {
    return <span>No posts have been added to this collection yet.</span>;
  }

  return (
    <div className={posts_style.posts}>
      {posts.map((post: Post, index: number) => {
        return <PostPreview post={post} key={index} />;
      })}
    </div>
  );
};

const AddCollaborator = (props: {
  collection: Collection;
  updateCollabs: Function;
}) => {
  const [username, setUsername] = useState<string>("");
  const [id, setID] = useState<number>(0);
  const [loading, setLoading] = useState<boolean>(false);
  const [users, setUsers] = useState<User[]>([]);
  const [existing_collabs, setExistingCollabs] = useState<User[]>([]);

  useEffect(() => {
    (async () => {
      const collabs = await getCollectionCollaborators(props.collection.id);
      setExistingCollabs(collabs);
    })();
  }, [props.collection.id]);

  const search = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    setLoading(true);
    const us = await searchUsers(username, id);
    setUsers(us);
    setLoading(false);
  };

  const updateCollabs = (collabs: User[]) => {
    const user_ids: number[] = [];
    for (let i = 0; i < collabs.length; i++) {
      user_ids.push(collabs[i].id);
    }
    props.updateCollabs(user_ids);
  };

  const addCollaborator = async (e: BaseSyntheticEvent, user_id: number) => {
    e.preventDefault();
    if (props.collection.creator === user_id) return;
    const add = await addCollaboratorToCollection(props.collection.id, user_id);
    if (add) {
      setUsers([]);
      const collabs = await getCollectionCollaborators(props.collection.id);
      updateCollabs(collabs);
      setExistingCollabs(collabs);
    }
  };

  const removeCollaborator = async (e: BaseSyntheticEvent, user_id: number) => {
    e.preventDefault();
    if (props.collection.creator === user_id) return;
    const remove = await removeCollaboratorFromCollection(
      props.collection.id,
      user_id
    );
    if (remove) {
      updateCollabs(existing_collabs.filter((v, i) => v.id !== user_id));
      setExistingCollabs((old) => old.filter((v, i) => v.id !== user_id));
    }
  };

  return (
    <>
      <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        <h2>Add Collaborator to Collection</h2>
        <section style={{ display: "flex", gap: "0.5rem" }}>
          <input
            style={{ flex: "1" }}
            type="text"
            placeholder="Username or ID"
          />
          <button onClick={search}>Search</button>
        </section>
        {loading ? (
          <span>Loading...</span>
        ) : (
          <>
            {users.map((user, index) => {
              return (
                <button
                  key={index}
                  style={{ padding: "0" }}
                  onClick={async (e: BaseSyntheticEvent) =>
                    await addCollaborator(e, user.id)
                  }
                >
                  <UserPreview user={user}></UserPreview>
                </button>
              );
            })}
          </>
        )}
        <h2>Existing Collaborators</h2>
        {existing_collabs.length <= 0 ? (
          <span>No collaborators have been added.</span>
        ) : (
          <>
            <span style={{ opacity: "0.5" }}>(Click to remove)</span>
            {existing_collabs.map((user, index) => {
              return (
                <button
                  key={index}
                  style={{ padding: "0" }}
                  onClick={async (e: BaseSyntheticEvent) =>
                    await removeCollaborator(e, user.id)
                  }
                >
                  <UserPreview user={user}></UserPreview>
                </button>
              );
            })}
          </>
        )}
      </div>
    </>
  );
};

const CollectionClient = (props: {
  collection: Collection;
  user: User | null;
}) => {
  const [show_add_collab, setShowAddCollab] = useState<boolean>(false);
  const [collabs, setCollabs] = useState<number[]>(props.collection.collaborators);

  const deleteColl = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const deletion = await deleteCollection(props.collection.id);
    if (deletion === null) {
      createNotification("Failed to delete collection.");
      return;
    }
    createNotification("Deleted collection!");
    window.location.href = `/user/${props.collection.creator}?view=collections`;
  };

  const addCollab = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    setShowAddCollab(true);
  };

  const is_creator = props.collection.creator === props.user?.id;

  return (
    <>
      {show_add_collab && (
        <Popup>
          <button
            onClick={() => setShowAddCollab(false)}
            style={{ mixBlendMode: "difference" }}
          >
            X
          </button>
          <AddCollaborator
            updateCollabs={(c: number[]) => {
              setCollabs(c);
            }}
            collection={props.collection}
          />
        </Popup>
      )}
      <div className={style.collection_header}>
        <h1>{props.collection.name}</h1>
        <p>{props.collection.description}</p>
        <LikeChip
          user={props.user}
          likes={props.collection.likes}
          dislikes={props.collection.dislikes}
          post_id={props.collection.id}
          post_type="collection"
        />
        <div
          style={{
            display: "flex",
            gap: "1rem",
            marginTop: "1rem",
            alignItems: "center",
          }}
        >
          <UserChip user_id={props.collection.creator} />
          {is_creator && (
            <>
              <button className="impact" onClick={addCollab}>
                Add Collaborator
              </button>
              <button className="impact" onClick={deleteColl}>
                Delete Collection
              </button>
            </>
          )}
        </div>
        {collabs.length >= 1 && (
          <div style={{ marginTop: "1rem" }}>
            <span style={{ opacity: "0.5" }}>(Collaborators)</span>
            <div
              style={{
                display: "flex",
                gap: "1rem",
                alignItems: "center",
              }}
            >
              {collabs.map((collab, index) => {
                return <UserChip user_id={collab} key={index} />;
              })}
            </div>
          </div>
        )}
      </div>
      <div>
        <Posts posts={props.collection.linked_posts} />
      </div>
    </>
  );
};

export default CollectionClient;
