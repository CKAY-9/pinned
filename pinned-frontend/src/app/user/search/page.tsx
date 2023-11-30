import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import UserSearchClient from "./client";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Search Users // Pinned",
    "description": "Search all public users on Pinned."
  }
}

const UserSearchPage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} /> 
      <main className="container">
        <UserSearchClient />
      </main>
      <Footer user={user} />
    </>
  );
}

export default UserSearchPage;
