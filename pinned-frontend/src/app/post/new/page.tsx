import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import { redirect } from "next/navigation";
import NewPostClient from "./client";

export const generateMetadata = (): Metadata => {
  return {
    "title": "New Post // Pinned",
    "description": "Create a new post to Pinned."
  }
}

const NewPostPage = async () => {
  const user = await getUserFromToken();

  if (user === null) {
    redirect("/user/login");
  }

  return (
    <>
      <Header user={user} />
      <main className="container">
        <NewPostClient user={user} />
      </main>
      <Footer user={user} />
    </>
  );
}

export default NewPostPage;
