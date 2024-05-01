import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import ExplorePostsClient from "./client";

export const generateMetadata = (): Metadata => {
  return {
    title: "Explore Users // Pinned",
    description: "Explore all the users on Pinned!",
  };
};

const PostExplorePage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user}></Header>
      <ExplorePostsClient />
      <Footer user={user}></Footer>
    </>
  );
};

export default PostExplorePage;
