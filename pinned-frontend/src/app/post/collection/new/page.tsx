import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import { redirect } from "next/navigation";
import NewCollectionClient from "./client";

export const generateMetadata = (): Metadata => {
  return {
    "title": "New Collection // Pinned",
    "description": "Create new collection on Pinned."
  }
}

const NewCollectionPage = async () => {
  const user = await getUserFromToken();

  if (user === null) {
    redirect("/user/login");
  }

  return (
    <>
      <Header user={user} />
      <main className="container">
        <NewCollectionClient />
      </main>
      <Footer user={user} />
    </>
  );
}

export default NewCollectionPage;
