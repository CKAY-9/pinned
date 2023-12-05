import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Explore Posts // Pinned",
    "description": "Explore all the public posts on Pinned."
  }
}

const PostExplorePage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
      </main>
      <Footer user={user} />
    </>
  );
}

export default PostExplorePage;
