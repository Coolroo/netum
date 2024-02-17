import NextAuth, { AuthOptions } from "next-auth";
import GoogleProvider from "next-auth/providers/google";
import { sign, verify } from "jsonwebtoken";

const JWT_SECRET = process.env.NEXTAUTH_SECRET;
if (!JWT_SECRET) {
  throw new Error("Missing env.NEXTAUTH_SECRET");
}

const authOptions: AuthOptions = {
  providers: [
    GoogleProvider({
      clientId: process.env.GOOGLE_CLIENT_ID ?? "",
      clientSecret: process.env.GOOGLE_CLIENT_SECRET ?? "",
    }),
  ],
};

const handler = NextAuth(authOptions);

export { handler as GET, handler as POST };
