"use client"

import { User } from "@/api/user/dto";
import style from "./settings.module.scss";
import { BaseSyntheticEvent, useState } from "react";
import { deleteUser, logoutUser, resetUser, updateUser } from "@/api/user/user.client";

const UserSettingsClient = (props: {
  user: User | null
}) => {
  const [bio, setBio] = useState<string>(props.user?.bio || "");

  const deleteAccount = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    await deleteUser();
  }
  
  const logout = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    logoutUser();
  }

  const resetAccount = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    await resetUser();
  }

  const updateAccount = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    await updateUser(bio);
  }

  return ( 
    <>
      <h1 style={{"marginBottom": "1rem"}}>Settings for {props.user?.username}</h1>
      <section className={style.options}>
        <h2>User Options</h2>
        <label>Bio</label>
        <textarea onChange={(e: BaseSyntheticEvent) => setBio(e.target.value)} placeholder="Bio" defaultValue={bio} cols={50} rows={10}></textarea>
        <button onClick={updateAccount}>Update</button>
      </section>
      <section className={`${style.unsafe_options} + ${style.options}`}>
        <h2>Dangerous Options</h2>
        <button className={style.unsafe} onClick={deleteAccount}>Delete Account</button>
        <button className={style.unsafe} onClick={logout}>Logout</button>
        <button className={style.unsafe} onClick={resetAccount}>Reset Account</button>
      </section>
    </>
  );
}

export default UserSettingsClient;
