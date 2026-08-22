export function CuteError({
  title,
  description,
}: {
  title: string;
  description: string;
}) {
  return (
    <div className="flex min-h-full flex-col items-center justify-center gap-6 p-8">
      <div className="card w-full max-w-xl bg-base-200/80 shadow-xl backdrop-blur-sm">
        <div className="card-body items-center gap-4 text-center">
          <h2 className="text-2xl font-bold tracking-tight text-red-400">{title}</h2>
          <p className="max-w-sm text-base-content/60">{description}</p>
        </div>
      </div>
    </div>
  );
}
