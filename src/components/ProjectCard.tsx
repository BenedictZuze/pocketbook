import React from "react";
import { useAtom } from "jotai";
import { Link } from "wouter";
import {
  Play,
  Square,
  CheckCircle as CircleCheck,
  Circle as CircleX,
  ExternalLink,
  Eye,
} from "lucide-react";
import { PocketBaseProject } from "../types";
import { projectsAtom } from "../store";
import {
  formatDate,
  getHealthColor,
  resumeProject,
  stopProject,
} from "../utils";

interface ProjectCardProps {
  project: PocketBaseProject;
}

export const ProjectCard: React.FC<ProjectCardProps> = ({ project }) => {
  const [projects, setProjects] = useAtom(projectsAtom);

  const toggleProjectStatus = async () => {
    if (project.status === "running") {
      await stopProject(project.id);
    } else {
      await resumeProject(project.id);
    }
    setProjects(
      projects.map((p) =>
        p.id === project.id
          ? {
              ...p,
              status: p.status === "running" ? "stopped" : "running",
              isHealthy: p.status === "stopped" ? true : p.isHealthy,
              lastStarted: p.status === "stopped" ? new Date() : p.lastStarted,
            }
          : p
      )
    );
  };

  const adminUrl = `http://127.0.0.1:${project.port}/_/`;

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center space-x-3 mb-3">
            <h3 className="text-lg font-semibold text-gray-900">
              {project.name}
            </h3>
            <span
              className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                project.status === "running"
                  ? "bg-green-100 text-green-800"
                  : "bg-gray-100 text-gray-800"
              }`}
            >
              {project.status}
            </span>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
            <div>
              <p className="text-sm text-gray-500">Port</p>
              <p className="text-sm font-medium text-gray-900">
                {project.port}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-500">Health</p>
              <div className="flex items-center space-x-1">
                {project.isHealthy ? (
                  <CircleCheck
                    className={`h-4 w-4 ${getHealthColor(project.isHealthy)}`}
                  />
                ) : (
                  <CircleX
                    className={`h-4 w-4 ${getHealthColor(project.isHealthy)}`}
                  />
                )}
                <span
                  className={`text-sm font-medium ${getHealthColor(
                    project.isHealthy
                  )}`}
                >
                  {project.isHealthy ? "Healthy" : "Unhealthy"}
                </span>
              </div>
            </div>
            {project.dataDirectory && (
              <div className="sm:col-span-2">
                <p className="text-sm text-gray-500">Data Directory</p>
                <p className="text-sm font-medium text-gray-900 font-mono break-words whitespace-normal">
                  {project.dataDirectory}
                </p>
              </div>
            )}
            <div className="sm:col-span-2">
              <p className="text-sm text-gray-500">Created</p>
              <p className="text-sm font-medium text-gray-900">
                {formatDate(new Date(project.createdAt))}
              </p>
            </div>
            {project.lastStarted && (
              <div className="sm:col-span-2">
                <p className="text-sm text-gray-500">Last Started</p>
                <p className="text-sm font-medium text-gray-900">
                  {formatDate(new Date(project.lastStarted))}
                </p>
              </div>
            )}
          </div>
        </div>
      </div>

      <div className="flex items-center justify-between pt-4 border-t border-gray-200">
        <div className="flex items-center space-x-2">
          <button
            onClick={toggleProjectStatus}
            className={`inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md transition-colors ${
              project.status === "running"
                ? "text-red-700 bg-red-100 hover:bg-red-200"
                : "text-green-700 bg-green-100 hover:bg-green-200"
            }`}
          >
            {project.status === "running" ? (
              <>
                <Square className="h-4 w-4 mr-2" />
                Stop
              </>
            ) : (
              <>
                <Play className="h-4 w-4 mr-2" />
                Start
              </>
            )}
          </button>

          {project.status === "running" && (
            <a
              href={adminUrl}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center px-3 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 transition-colors"
            >
              <ExternalLink className="h-4 w-4 mr-2" />
              Admin UI
            </a>
          )}
        </div>

        <Link href={`/project/${project.id}`}>
          <a className="inline-flex items-center px-3 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 transition-colors">
            <Eye className="h-4 w-4 mr-2" />
            Details
          </a>
        </Link>
      </div>
    </div>
  );
};
