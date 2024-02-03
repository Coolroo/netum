"use client"

import { usePathname } from "next/navigation"

export default function PagePath() {
    const pathname = usePathname();
    return (<div>
        {pathname}
    </div>)
}