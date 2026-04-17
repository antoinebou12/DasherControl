import { isAxiosError } from "axios";

/** Human-readable message for axios/network errors (avoids "[object Object]"). */
export function formatAxiosError(error: unknown): string {
  if (isAxiosError(error)) {
    const status = error.response?.status;
    const data = error.response?.data;
    if (typeof data === "string" && data.trim()) return data;
    if (data && typeof data === "object") {
      const rec = data as Record<string, unknown>;
      if (rec.error != null) return String(rec.error);
      if (rec.message != null) return String(rec.message);
      try {
        return JSON.stringify(data);
      } catch {
        return "Request failed.";
      }
    }
    if (status === 500) return "Server error. Try again later.";
    if (status === 401) {
      // Body may be missing; object branch above usually already returned { error }.
      return "Unauthorized.";
    }
    if (status === 409) return "Resource conflict.";
    if (status) return `Request failed (${status}).`;
    if (error.message) return error.message;
  }
  if (error instanceof Error) return error.message;
  return String(error ?? "Something went wrong.");
}
