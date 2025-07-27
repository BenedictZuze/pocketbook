import React from "react";
import { useAtomValue, useSetAtom } from "jotai";
import { useRoute, Link } from "wouter";
import {
  ArrowLeft,
  CheckCircle as CircleCheck,
  Circle as CircleX,
  ExternalLink,
} from "lucide-react";
import { projectsAtom, selectedProjectIdAtom } from "../store";
import { formatDate, getHealthColor } from "../utils";

export const ProjectDetails: React.FC = () => {
  const [, params] = useRoute("/project/:id");
  const projects = useAtomValue(projectsAtom);
  const setSelectedProjectId = useSetAtom(selectedProjectIdAtom);

  const project = projects.find((p) => p.id === params?.id);

  React.useEffect(() => {
    if (params?.id) {
      setSelectedProjectId(params.id);
    }
    return () => setSelectedProjectId(null);
  }, [params?.id, setSelectedProjectId]);

  if (!project) {
    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold text-gray-900 mb-2">
          Project Not Found
        </h2>
        <p className="text-gray-600 mb-6">
          The requested project could not be found.
        </p>
        <Link href="/">
          <a className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors">
            <ArrowLeft className="h-4 w-4 mr-2" />
            Back to Projects
          </a>
        </Link>
      </div>
    );
  }

  const adminUrl = `http://127.0.0.1:${project.port}/_/`;

  return (
    <div>
      <div className="mb-6">
        <Link href="/">
          <a className="inline-flex items-center text-sm text-gray-500 hover:text-gray-700 transition-colors mb-4">
            <ArrowLeft className="h-4 w-4 mr-1" />
            Back to Projects
          </a>
        </Link>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-gray-900 mb-2">
              {project.name}
            </h1>
            <div className="flex items-center space-x-4">
              <span
                className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                  project.status === "running"
                    ? "bg-green-100 text-green-800"
                    : "bg-gray-100 text-gray-800"
                }`}
              >
                {project.status}
              </span>
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
          </div>
          <div className="flex items-center space-x-3">
            {project.status === "running" && (
              <a
                href={adminUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 transition-colors"
              >
                <ExternalLink className="h-4 w-4 mr-2" />
                Admin UI
              </a>
            )}
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Project Metadata */}
        <div className="lg:col-span-1">
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">
              Project Information
            </h2>
            <dl className="space-y-4">
              <div>
                <dt className="text-sm text-gray-500">Port</dt>
                <dd className="text-sm font-medium text-gray-900">
                  {project.port}
                </dd>
              </div>
              <div>
                <dt className="text-sm text-gray-500">Status</dt>
                <dd className="text-sm font-medium text-gray-900 capitalize">
                  {project.status}
                </dd>
              </div>
              <div>
                <dt className="text-sm text-gray-500">Health Status</dt>
                <dd
                  className={`text-sm font-medium ${getHealthColor(
                    project.isHealthy
                  )}`}
                >
                  {project.isHealthy ? "Healthy" : "Unhealthy"}
                </dd>
              </div>
              {project.dataDirectory && (
                <div>
                  <dt className="text-sm text-gray-500">Data Directory</dt>
                  <dd className="text-sm font-medium text-gray-900 font-mono break-all">
                    {project.dataDirectory}
                  </dd>
                </div>
              )}
              <div>
                <dt className="text-sm text-gray-500">Created</dt>
                <dd className="text-sm font-medium text-gray-900">
                  {formatDate(project.createdAt)}
                </dd>
              </div>
              {project.lastStarted && (
                <div>
                  <dt className="text-sm text-gray-500">Last Started</dt>
                  <dd className="text-sm font-medium text-gray-900">
                    {formatDate(project.lastStarted)}
                  </dd>
                </div>
              )}
            </dl>
          </div>
        </div>

        {/* Logs */}
        <div className="lg:col-span-2">
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">
              Recent Logs
            </h2>

            {project.logs.length === 0 ? (
              <div className="text-center py-8">
                <p className="text-gray-500">No logs available</p>
                <p className="text-sm text-gray-400 mt-1">
                  Logs will appear here when the project is running
                </p>
              </div>
            ) : (
              <div className="space-y-3 max-h-96 overflow-y-auto">
                {project.logs.map((log) => (
                  <div
                    key={log.id}
                    className="flex items-start space-x-3 p-3 bg-gray-50 rounded-md"
                  >
                    <div className="flex-shrink-0">
                      <span
                        className={`inline-flex items-center px-2 py-1 rounded text-xs font-medium ${
                          log.level === "error"
                            ? "bg-red-100 text-red-800"
                            : log.level === "warn"
                            ? "bg-yellow-100 text-yellow-800"
                            : "bg-blue-100 text-blue-800"
                        }`}
                      >
                        {log.level.toUpperCase()}
                      </span>
                    </div>
                    <div className="flex-1 min-w-0">
                      <p className="text-sm text-gray-900 font-mono">
                        {log.message}
                      </p>
                      <p className="text-xs text-gray-500 mt-1">
                        {formatDate(log.timestamp)}
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
