import { ArrowLeft } from "lucide-react";
import { Link } from "wouter";

export default function NotFound() {
  return (
    <main className="notfound-shell">
      <div className="notfound-inner">
        <p className="eyebrow"><span className="signal-dot" />ERROR / 404</p>
        <h1>404</h1>
        <p className="notfound-lead">
          The page you are looking for does not exist. It may have been moved or removed.
        </p>
        <Link href="/" className="primary-action">
          <ArrowLeft size={17} />
          Back to TurkmenAI Local
        </Link>
      </div>
    </main>
  );
}
