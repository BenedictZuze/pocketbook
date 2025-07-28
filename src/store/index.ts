import { atom } from "jotai";
import { PocketBaseProject } from "../types";
import PocketBase from "pocketbase";

// Mock data for demonstration
// const mockLogs: LogEntry[] = [
//   {
//     id: "1",
//     timestamp: new Date(Date.now() - 3600000),
//     level: "info",
//     message: "Server started successfully on port 8090",
//   },
//   {
//     id: "2",
//     timestamp: new Date(Date.now() - 1800000),
//     level: "info",
//     message: "Database connection established",
//   },
//   {
//     id: "3",
//     timestamp: new Date(Date.now() - 900000),
//     level: "warn",
//     message: "High memory usage detected",
//   },
// ];

const initialProjects: PocketBaseProject[] = [
  {
    id: "1",
    name: "E-commerce API",
    port: 8090,
    status: "running",
    isHealthy: true,
    pid: "12345",
    dataDirectory: "/data/ecommerce",
    createdAt: new Date(Date.now() - 86400000 * 7),
    lastStarted: new Date(Date.now() - 3600000),
    // logs: mockLogs,
  },
  {
    id: "2",
    name: "Blog Backend",
    port: 8091,
    status: "stopped",
    isHealthy: false,
    pid: "12345",
    dataDirectory: "/data/blog",
    createdAt: new Date(Date.now() - 86400000 * 3),
    // logs: [],
  },
  {
    id: "3",
    name: "User Management",
    port: 8092,
    status: "running",
    isHealthy: true,
    pid: "12345",
    createdAt: new Date(Date.now() - 86400000),
    lastStarted: new Date(Date.now() - 1800000),
    // logs: mockLogs.slice(0, 2),
  },
];

const pb = new PocketBase("http://127.0.0.1:8090");
const db = await pb
  .collection("users")
  .authWithPassword(
    import.meta.env.VITE_MASTER_EMAIL,
    import.meta.env.VITE_MASTER_PASSWORD
  );
export const pbAtom = atom<PocketBase>(pb);
export const dbAtom = atom(db);

const initialDbProjects = await pb
  .collection("projects")
  .getFullList<PocketBaseProject>();
export const projectsAtom = atom<PocketBaseProject[]>(
  initialDbProjects || initialProjects
);
export const selectedProjectIdAtom = atom<string | null>(null);

// Derived atom for selected project
export const selectedProjectAtom = atom((get) => {
  const projects = get(projectsAtom);
  const selectedId = get(selectedProjectIdAtom);
  return projects.find((p) => p.id === selectedId) || null;
});
