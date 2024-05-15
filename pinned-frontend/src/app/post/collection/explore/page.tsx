import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import ExploreCollectionsClient from "./client";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Explore Collections // Pinned",
    "description": "Explore all the public collections on Pinned."
  }
}

const PostExplorePage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <ExploreCollectionsClient />
      <Footer user={user} />
    </>
  );
}

export default PostExplorePage;
