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
      <section className={style.unsafe_options}>
        <button className={style.unsafe} onClick={deleteAccount}>Delete Account</button>
        <button className={style.unsafe}>Logout</button>
        <button className={style.unsafe}>Reset Account</button>
      </section>
    </>
  );
}

export default UserSettingsClient;
