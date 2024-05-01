"use client";

import { useEffect, useState } from "react";
import style from "./explore.module.scss";
import { User } from "@/api/user/dto";
import { getExploreUsers } from "@/api/user/user.client";
import Link from "next/link";

const ExploreUser = (props: { user: User }) => {
  return (
    <Link
      href={`/user/${props.user.id}`}
      className={style.item}
      style={{ background: `url(${props.user.avatar})`, color: "white" }}
    >
      <div className={style.content}>
        <h1 style={{ fontSize: "4rem" }}>{props.user.username}</h1>
        <span>{props.user.bio}</span>
      </div>
    </Link>
  );
};

const ExploreUsersClient = () => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      const us = await getExploreUsers();
      setUsers((old) => old.concat(us));
      setLoading(false);
    })();
  }, []);

  return (
    <div className={style.explore}>
      <div className={style.item} style={{ padding: "1rem" }}>
        <h1 style={{ fontSize: "4rem" }}>EXPLORE USERS</h1>
        <span>Find users on Pinned and what they have to show.</span>
      </div>
      {loading && (
        <div
          className={style.item}
          style={{
            backgroundColor: "var(--primary)",
            color: "white",
            padding: "1rem",
          }}
        >
          <h1 style={{ fontSize: "4rem" }}>LOADING...</h1>
          <span>Please wait while we load some users for you...</span>
        </div>
      )}
      {users.length <= 0 && !loading && (
        <div
          className={style.item}
          style={{
            backgroundColor: "var(--primary)",
            color: "white",
            padding: "1rem",
          }}
        >
          <h1 style={{ fontSize: "4rem" }}>NO USERS FOUND</h1>
          <span>Failed to get any users, strange...</span>
        </div>
      )}
      {users.length >= 1 && !loading && (
        <>
          {users.map((val, index) => {
            return <ExploreUser user={val} key={index}></ExploreUser>;
          })}
        </>
      )}
    </div>
  );
};

export default ExploreUsersClient;
