import { getPostFromID } from "@/api/post/post"
import { getUserFromToken } from "@/api/user/user"
import Footer from "@/components/footer/footer"
import Header from "@/components/header/header"
import { Metadata } from "next"
import { redirect } from "next/navigation"
import PostClient from "./client"

export const generateMetadata = async ({params}: {
  params: {
    id: string
  }
}): Promise<Metadata> => {
  const post_id: number = Number.parseInt(params.id);
  const post = await getPostFromID(post_id);
  if (post === null) {
    return {
      "title": `Invalid Post // Pinned`,
      "description": `View all public posts on Pinned.`
    }
  }
  return {
    "title": `${post.title} // Pinned`,
    "description": `View post on Pinned. ${post.description}`
  }
}

const PostPage = async ({params}: {
  params: {
    id: string
  }
}) => {
  const post_id: number = Number.parseInt(params.id);
  const post = await getPostFromID(post_id);

  if (post === null) {
    redirect("/post/search");
  }

  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
        <PostClient post={post} user={user} />
      </main>
      <Footer user={user} />
    </>
  );
}

export default PostPage;
