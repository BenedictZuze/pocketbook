import { PocketBaseProject, NewProjectData } from "../types";
import { invoke } from "@tauri-apps/api/core";

export const generateProjectId = (): string => {
  return Math.random().toString(36).substr(2, 9);
};

export const getNextAvailablePort = (projects: PocketBaseProject[]): number => {
  const usedPorts = projects.map((p) => p.port);
  let port = 8090;
  while (usedPorts.includes(port)) {
    port++;
  }
  return port;
};

export const createProject = async (
  data: NewProjectData,
  existingProjects: PocketBaseProject[]
): Promise<PocketBaseProject> => {
  await invoke("start_pocketbase_instance", { projectName: data.name });
  return {
    id: generateProjectId(),
    name: data.name,
    port: getNextAvailablePort(existingProjects),
    status: "stopped",
    isHealthy: false,
    pid: "",
    dataDirectory: data.dataDirectory,
    createdAt: new Date(),
    lastStarted: new Date(),
    // logs: [],
  };
};

export const stopProject = async (projectName: string): Promise<void> => {
  return await invoke("stop_pocketbase_instance", { projectName });
};

export const resumeProject = async (projectName: string): Promise<void> => {
  return await invoke("resume_pocketbase_instance", { projectName });
};

export const formatDate = (date: Date): string => {
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
};

export const getStatusColor = (status: "running" | "stopped"): string => {
  return status === "running" ? "text-green-600" : "text-gray-500";
};

export const getHealthColor = (isHealthy: boolean): string => {
  return isHealthy ? "text-green-500" : "text-red-500";
};
