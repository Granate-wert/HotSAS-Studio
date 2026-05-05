import { backend } from "../api";

export type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

export interface LogEntry {
  id: number;
  timestamp: string;
  level: LogLevel;
  message: string;
  source: "frontend" | "backend";
}

const MAX_ENTRIES = 2000;

let nextId = 1;
const entries: LogEntry[] = [];
const listeners = new Set<() => void>();

function now(): string {
  const d = new Date();
  return `${d.getHours().toString().padStart(2, "0")}:${d.getMinutes().toString().padStart(2, "0")}:${d.getSeconds().toString().padStart(2, "0")}.${d.getMilliseconds().toString().padStart(3, "0")}`;
}

function push(entry: LogEntry) {
  entries.push(entry);
  if (entries.length > MAX_ENTRIES) {
    entries.splice(0, entries.length - MAX_ENTRIES);
  }
  listeners.forEach((cb) => cb());
}

function log(level: LogLevel, message: string, source: "frontend" | "backend" = "frontend") {
  const entry: LogEntry = {
    id: nextId++,
    timestamp: now(),
    level,
    message,
    source,
  };
  push(entry);

  const consoleMessage = `[${entry.timestamp}] [${source}] ${level.toUpperCase()}: ${message}`;
  switch (level) {
    case "error":
      console.error(consoleMessage);
      break;
    case "warn":
      console.warn(consoleMessage);
      break;
    case "debug":
      console.debug(consoleMessage);
      break;
    case "trace":
      console.trace(consoleMessage);
      break;
    default:
      console.log(consoleMessage);
  }

  try {
    void backend.writeLog(level, message).catch(() => {
      // Ignore write_log failures to avoid infinite loops
    });
  } catch {
    // Ignore synchronous failures from Tauri bridge
  }
}

export const logger = {
  trace: (message: string) => log("trace", message),
  debug: (message: string) => log("debug", message),
  info: (message: string) => log("info", message),
  warn: (message: string) => log("warn", message),
  error: (message: string) => log("error", message),

  pushBackendLog: (level: LogLevel, message: string) => {
    push({
      id: nextId++,
      timestamp: now(),
      level,
      message,
      source: "backend",
    });
  },

  getEntries: (): readonly LogEntry[] => entries,

  subscribe: (callback: () => void) => {
    listeners.add(callback);
    return () => listeners.delete(callback);
  },

  clear: () => {
    entries.length = 0;
    listeners.forEach((cb) => cb());
  },

  exportText: (): string => {
    return entries
      .map((e) => `[${e.timestamp}] [${e.source}] ${e.level.toUpperCase()}: ${e.message}`)
      .join("\n");
  },
};
