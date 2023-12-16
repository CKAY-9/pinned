"use client"

import { User } from "@/api/user/dto";
import style from "./settings.module.scss";
import { BaseSyntheticEvent } from "react";
import { deleteUser, logoutUser, resetUser } from "@/api/user/user.client";

const UserSettingsClient = (props: {
  user: User | null
}) => {

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

  return ( 
    <>
      <h1 style={{"marginBottom": "1rem"}}>User Settings for {props.user?.username}</h1>
      <section className={`${style.unsafe_options} + ${style.options}`}>
        <button className={style.unsafe} onClick={deleteAccount}>Delete Account</button>
        <button className={style.unsafe} onClick={logout}>Logout</button>
        <button className={style.unsafe} onClick={resetAccount}>Reset Account</button>
      </section>
    </>
  );
}

export default UserSettingsClient;
