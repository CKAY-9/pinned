import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Explore Posts // Pinned",
    "description": "Explore all public posts on Pinned."
  }
}

const PostExplorePage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} /> 
      <main className="container">
        <h1>Explore Posts</h1>
      </main>
      <Footer user={user} />
    </>
  );
}

export default PostExplorePage;
