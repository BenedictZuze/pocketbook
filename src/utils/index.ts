import { PocketBaseProject, NewProjectData } from "../types";

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

export const createProject = (
  data: NewProjectData,
  existingProjects: PocketBaseProject[]
): PocketBaseProject => {
  return {
    id: generateProjectId(),
    name: data.name,
    port: getNextAvailablePort(existingProjects),
    status: "stopped",
    isHealthy: false,
    dataDirectory: data.dataDirectory,
    createdAt: new Date(),
    logs: [],
  };
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
