"use client";
import StatBar from "./components/statbar";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "./components/Catalyst/table";

export default function Home() {
  let users = [
    {
      handle: "Clown",
      name: "jeff",
      email: "came@game.lame",
      access: "Groovy",
    },
  ];
  return (
    <>
      <ul role="list" className="divide-y divide-gray-200">
        <li key={"statbar"} className="py-4">
          <StatBar
            stats={[
              { name: "New Scans", stat: "50" },
              { name: "Books", stat: "20" },
              { name: "Games", stat: "10" },
            ]}
          />
        </li>
        <li key={"new_section"} className="py-4">
          <div className="overflow-hidden rounded-lg bg-white px-4 py-5 drop-shadow shadow-slate-300 sm:p-6 ">
            <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 text-center">
              <h1 className="text-3xl font-bold leading-tight tracking-tight text-gray-900">
                New Scans
              </h1>
            </div>
            <Table bleed>
              <TableHead>
                <TableRow>
                  <TableHeader>Name</TableHeader>
                  <TableHeader>Email</TableHeader>
                  <TableHeader>Role</TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {users.map((user) => (
                  <TableRow key={user.handle}>
                    <TableCell className="font-medium">{user.name}</TableCell>
                    <TableCell>{user.email}</TableCell>
                    <TableCell className="text-zinc-500">
                      {user.access}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        </li>
      </ul>
    </>
  );
}
