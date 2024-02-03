export default function PaddedContent({children}: {children: React.ReactNode}) {
    return <div className="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <section>{children}</section>
        </div>
  }