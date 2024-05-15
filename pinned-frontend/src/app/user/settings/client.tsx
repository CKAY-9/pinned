"use client"

import { User } from "@/api/user/dto";
import style from "./settings.module.scss";
import { BaseSyntheticEvent, useState } from "react";
import { deleteUser, logoutUser, resetUser, updateUser } from "@/api/user/user.client";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { createNotification } from "@/utils/notification";

const UserSettingsClient = (props: {
  user: User | null
}) => {
  const router = useRouter();
  const [bio, setBio] = useState<string>(props.user?.bio || "");

  const deleteAccount = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    createNotification("Deleting account...")
    await deleteUser();
  }
  
  const logout = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    createNotification("Logging out...")
    logoutUser();
  }

  const resetAccount = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    await resetUser();
    createNotification("Reset user data.")
  }

  const updateAccount = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const u = await updateUser(bio);
    if (u !== null) {
      createNotification("Updated account.")
    } else {
      createNotification("Failed to update account.")
    }
  }

  return ( 
    <>
      <button className="impact" style={{"marginBottom": "1rem", "width": "fit-content"}} onClick={() => router.back()}>Back</button>
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
