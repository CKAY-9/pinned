import { getUserFromToken } from "@/api/user/user"
import Footer from "@/components/footer/footer"
import Header from "@/components/header/header"
import { redirect } from "next/navigation"
import UserSettingsClient from "./client"
import { Metadata } from "next"

export const generateMetadata = (): Metadata => {
  return {
    "title": "User Settings // Pinned",
    "description": "Manange your Pinned account and settings."
  }
}

const UserSettingsPage = async () => {
  const user = await getUserFromToken();

  if (user === null) {
    redirect("/user/login");
  }

  return (
    <>
      <Header user={user} />
      <main className="container">
        <UserSettingsClient user={user} />
      </main>
      <Footer user={user} />
    </>
  )
}

export default UserSettingsPage;
