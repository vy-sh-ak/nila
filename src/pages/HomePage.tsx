import { ArrowRight, Bot, MessageSquare, Sparkles } from "lucide-react";

export default function HomePage() {
  return (
    <div className="flex min-h-full flex-col items-center justify-center gap-6 p-8">
      <div className="card w-full max-w-xl bg-base-200/80 shadow-xl backdrop-blur-sm">
        <div className="card-body gap-6">
          <div className="flex flex-col items-start gap-4">
            <div className="badge badge-soft gap-1 text-base-content/70">
              <Sparkles className="h-3.5 w-3.5" />
              Personal assistant
            </div>
            <h1 className="text-3xl font-bold tracking-tight">
              Welcome to Nila
            </h1>
            <p className="text-base-content/60">
              Your personal assistant workspace. Pick a model to start a
              conversation, or explore what's available.
            </p>
          </div>

          <div className="flex flex-col gap-3 sm:flex-row">
            <button className="btn btn-primary flex-1">
              <MessageSquare className="h-4 w-4" />
              Start a conversation
            </button>
            <button className="btn btn-soft flex-1">
              <Bot className="h-4 w-4" />
              Browse models
              <ArrowRight className="h-4 w-4" />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}