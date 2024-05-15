import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import PostSearchClient from "./client";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Search Posts // Pinned",
    "description": "Search all public posts on Pinned."
  }
}

const PostSearchPage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} /> 
      <main className="container">
        <PostSearchClient user={user} />
      </main>
      <Footer user={user} />
    </>
  );
}

export default PostSearchPage;
