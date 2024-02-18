import NextAuth, { AuthOptions } from "next-auth";
import GoogleProvider from "next-auth/providers/google";
import { sign, verify } from "jsonwebtoken";

const JWT_SECRET = process.env.NEXTAUTH_SECRET;
if (!JWT_SECRET) {
  throw new Error("Missing env.NEXTAUTH_SECRET");
}

export const authOptions: AuthOptions = {
  providers: [
    GoogleProvider({
      clientId: process.env.GOOGLE_CLIENT_ID ?? "",
      clientSecret: process.env.GOOGLE_CLIENT_SECRET ?? "",
    }),
  ],
  jwt: {
    encode: async ({ token }) => {
      if (!token) return "";

      if (token.exp === undefined) {
        return sign(token, JWT_SECRET, {
          algorithm: "HS256",
          expiresIn: "24h",
        });
      }

      return sign(token, JWT_SECRET, { algorithm: "HS256" });
    },
    // @ts-expect-error
    decode: async ({ token: tokenStr }) => {
      console.log("here");
      console.log(tokenStr);
      if (!tokenStr) return null;
      return verify(tokenStr, JWT_SECRET);
    },
  },
  session: {
    strategy: "jwt",
    maxAge: 30 * 24 * 60 * 60, // 30 days
  },
  /*callbacks: {
    jwt: async ({ token, user, account, profile }) => {
      console.log("here");
      console.log(token);
      console.log(user);
      const isSignIn = user ? true : false;
      // Add auth_time to token on signin in
      if (isSignIn) {
        token.auth_time = Math.floor(Date.now() / 1000);
      }
      return Promise.resolve(token);
    },
    session: async ({ session, token, user }) => {
      console.log("here2");
      console.log(token);
      console.log(session);
      if (!session?.user || !token?.account) {
        return session;
      }

      session.user.id = token.sub;
      session.user.name = token.name;
      session.user.image = token.picture;
      session.user.email = token.email;
      return session;
    },
  },*/
  secret: process.env.NEXTAUTH_SECRET,
};

const handler = NextAuth(authOptions);

export { handler as GET, handler as POST };
