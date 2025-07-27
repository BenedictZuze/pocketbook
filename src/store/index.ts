import { atom } from "jotai";
import { PocketBaseProject, LogEntry } from "../types";

// Mock data for demonstration
const mockLogs: LogEntry[] = [
  {
    id: "1",
    timestamp: new Date(Date.now() - 3600000),
    level: "info",
    message: "Server started successfully on port 8090",
  },
  {
    id: "2",
    timestamp: new Date(Date.now() - 1800000),
    level: "info",
    message: "Database connection established",
  },
  {
    id: "3",
    timestamp: new Date(Date.now() - 900000),
    level: "warn",
    message: "High memory usage detected",
  },
];

const initialProjects: PocketBaseProject[] = [
  {
    id: "1",
    name: "E-commerce API",
    port: 8090,
    status: "running",
    isHealthy: true,
    dataDirectory: "/data/ecommerce",
    createdAt: new Date(Date.now() - 86400000 * 7),
    lastStarted: new Date(Date.now() - 3600000),
    logs: mockLogs,
  },
  {
    id: "2",
    name: "Blog Backend",
    port: 8091,
    status: "stopped",
    isHealthy: false,
    dataDirectory: "/data/blog",
    createdAt: new Date(Date.now() - 86400000 * 3),
    logs: [],
  },
  {
    id: "3",
    name: "User Management",
    port: 8092,
    status: "running",
    isHealthy: true,
    createdAt: new Date(Date.now() - 86400000),
    lastStarted: new Date(Date.now() - 1800000),
    logs: mockLogs.slice(0, 2),
  },
];

export const projectsAtom = atom<PocketBaseProject[]>(initialProjects);
export const selectedProjectIdAtom = atom<string | null>(null);

// Derived atom for selected project
export const selectedProjectAtom = atom((get) => {
  const projects = get(projectsAtom);
  const selectedId = get(selectedProjectIdAtom);
  return projects.find((p) => p.id === selectedId) || null;
});
