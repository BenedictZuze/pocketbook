export interface PocketBaseProject {
  id: string;
  name: string;
  port: number;
  status: "running" | "stopped";
  isHealthy: boolean;
  dataDirectory?: string;
  createdAt: Date;
  lastStarted?: Date;
  logs: LogEntry[];
}

export interface LogEntry {
  id: string;
  timestamp: Date;
  level: "info" | "warn" | "error";
  message: string;
}

export interface NewProjectData {
  name: string;
  dataDirectory?: string;
}
